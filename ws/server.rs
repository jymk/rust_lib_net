use std::{
    collections::BTreeMap,
    io::{BufReader, Read},
    net::TcpStream,
    time::Duration,
};

use bytes::BytesMut;
use sha1::{Digest, Sha1};

use super::frame::_Frame;
use crate::http::server::{back_with_header, write_msg};
use crate::http::{req::*, rsp::*};
use crate::tcp::server::*;
use common::{
    error,
    errs::SResult,
    status::LoopStatus,
    time::{self as common_time, now_drt},
    trace,
};

type Handler = fn(&BytesMut) -> Option<Vec<u8>>;

static DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// 默认addr: localhost:7880
/// 默认超时: 30s
/// 这里不需要指定path，因为每次启动都是不能占用同样端口的，并且每次启动只能有一个websocket服务
/// 暂不支持分片传输
/// 心跳时不处理消息
/// 若opcode传值大于2^4 -1 或者 回包为None，则置为0xa(pongs)
#[derive(Clone)]
pub struct WSServer {
    pub tcp_svr: TcpServer,
    _expire: Duration,
    _timeout: Duration,
    _status: WSStatus,
    pub handler: Handler,
}

impl WSServer {
    pub fn with_timeout(&mut self, timeout: Duration) -> &mut Self {
        self._timeout = timeout;
        self._update_expire_with_timeout(timeout);
        self
    }

    fn _update_expire_with_timeout(&mut self, timeout: Duration) {
        self._expire = common_time::now_drt() + timeout;
    }

    fn _on_msg(&mut self, stream: &TcpStream) -> bool {
        // 接收到消息后更新expire
        self._update_expire_with_timeout(self._timeout);

        let mut br = BufReader::new(stream);

        let frame = _read_msg(&mut br, self);
        if frame.is_none() {
            return false;
        }
        let frame = frame.unwrap();
        match frame._opcode {
            0x8 => {
                return false;
            }
            0x9 => {
                _write_msg(stream, 0xa, b"");
                return true;
            }
            _ => {}
        }

        let rsp = (self.handler)(&frame._data);
        let mut opcode = 0x1;
        let rsp_msg;
        if rsp.is_none() {
            opcode = 0xa;
            rsp_msg = Vec::default();
        } else {
            rsp_msg = rsp.unwrap();
        }
        // 向前端写数据
        _write_msg(stream, opcode, &rsp_msg);
        true
    }

    fn _on_start(&mut self, stream: &TcpStream) {
        let mut br = BufReader::new(stream);
        let buf = crate::http::header::read_header(&mut br);
        // trace!("\nreq={:?}", buf);
        //读取header
        let req = HttpRequest::new(&buf);
        if req.is_err() {
            crate::http::server::back(stream, StatusCode::InternalServerError, req.unwrap_err());
            return;
        }
        let mut req = req.unwrap();
        let req_header = req.get_header().clone();
        let method = req.get_method();
        //读取body
        let code = req.body_mut().obtain_body(&mut br, &req_header, method);
        if code.is_err() {
            crate::http::server::back(stream, StatusCode::BadRequest, code.unwrap_err());
            return;
        }

        trace!("header={:?}", req.get_header());
        let mut rsp = HttpResponse::default();
        // 计算
        let wskey = req.get_header().get("Sec-WebSocket-Key");
        let wsversion = req.get_header().get("Sec-WebSocket-Version");
        if let (Some(key), Some(ver)) = (wskey, wsversion) {
            let mut hasher = Sha1::new();
            hasher.update(key.to_string() + super::MAGIC);
            let res = hasher.finalize();
            rsp.set_header(
                "Sec-WebSocket-Accept",
                &common::base64::base64_encode(&res[..]),
            );
            rsp.set_header("Sec-WebSocket-Version", ver);
            rsp.set_header("Connection", "Upgrade");
            rsp.set_header("Upgrade", "websocket");
        }
        trace!("header={:?}", rsp);

        self._status = WSStatus::Handling;
        // 回包
        back_with_header(
            stream,
            StatusCode::SwitchingProtocols,
            rsp.get_body().to_string(),
            rsp.headers_mut(),
        );
    }
}

impl Server for WSServer {
    fn start(self) {
        let mut this = self.clone();
        self.tcp_svr.start(move |stream| {
            this._status = WSStatus::Start;
            loop {
                match this._status {
                    WSStatus::Start => this._on_start(stream),
                    WSStatus::End => break,
                    WSStatus::Handling => {
                        if !this._on_msg(stream) {
                            break;
                        }
                    }
                }
                // 超时后关闭连接
                if common_time::now_drt() > this._expire {
                    break;
                }
            }
            LoopStatus::Break
        });
    }
}

fn _read_msg(br: &mut BufReader<&TcpStream>, server: &mut WSServer) -> Option<_Frame> {
    let mut frame = _Frame::default();
    let mut flag = false;
    loop {
        let mut tmp = _Frame::default();
        let res = _read_head(br, &mut tmp);
        if res.is_err() {
            error!("_read_head: err={:?}", res.unwrap_err());
            server._status = WSStatus::End;
            return None;
        }
        match tmp._opcode {
            0x8 => {
                server._status = WSStatus::End;
                return None;
            }
            0x9 => break,
            _ => {}
        }
        if !flag {
            flag = true;
            frame = tmp.clone();
        }
        let res = _read_data(br, &mut tmp);
        if res.is_err() {
            error!("_read_data: err={:?}", res.unwrap_err());
            server._status = WSStatus::End;
            return None;
        }
        // trace!("single_frame={:?}", tmp);
        frame._data.extend(tmp._data);
        if tmp._fin {
            break;
        }
    }
    Some(frame)
}

/// 写数据
fn _write_msg(stream: &TcpStream, mut opcode: u8, msg: &[u8]) {
    if opcode as u32 > 2u32.pow(4) - 1 {
        opcode = 0xa;
    }
    let mut rsp = Vec::default();
    // fin、rsv1/2/3、opcode
    let data = u8::from_str_radix(
        &format!("{:0>1b}{:0>1b}{:0>1b}{:0>1b}{:0>4b}", 1, 0, 0, 0, opcode),
        2,
    )
    .unwrap();
    rsp.push(data);

    // mask、payload_len
    let len = msg.len();
    let payload_len;
    if len <= 125 {
        payload_len = len;
    } else if len <= 65535 {
        payload_len = 126;
    } else {
        payload_len = 127;
    }
    let data = u8::from_str_radix(&format!("{:0>1b}{:0>7b}", 0, payload_len), 2).unwrap();
    rsp.push(data);

    // 放0个字节或2个字节或8个字节，无mask_key
    match payload_len {
        126 => {
            let data = u16::from_str_radix(&format!("{:0>16b}", len), 2).unwrap();
            rsp.push((data & 0xFF) as u8);
            rsp.push((data >> 8) as u8);
        }
        127 => {
            let data = u64::from_str_radix(&format!("{:0>64b}", len), 2).unwrap();
            rsp.push((data & 0xFFFF) as u8);
            rsp.push((data & 0xFFF) as u8);
            rsp.push((data & 0xFF) as u8);
            rsp.push((data >> 56) as u8);
        }
        _ => {}
    }

    // msg
    rsp.extend_from_slice(msg);
    // trace!("rsp={:?}", rsp);
    write_msg(stream, &rsp);
}

/// 读取数据体
fn _read_data(br: &mut BufReader<&TcpStream>, frame: &mut _Frame) -> SResult<()> {
    let mut res = _read_frame(br, frame._payload_len as usize)?;
    if frame._mask {
        _unmask(&mut res, &frame._mask_key);
    }
    frame._data = BytesMut::from_iter(res);
    Ok(())
}

/// 读取头
fn _read_head(br: &mut BufReader<&TcpStream>, frame: &mut _Frame) -> SResult<()> {
    // 读取FIN、rsv1、rsv2、rsv3、opcode
    let res = _read_frame(br, 1)?;
    let data = res[0];

    // 2^7
    frame._fin = data & 0x80 == 0x80;
    // 2^6
    frame._rsv1 = data & 0x40 == 0x40;
    // 2^5
    frame._rsv2 = data & 0x20 == 0x20;
    // 2^4
    frame._rsv3 = data & 0x10 == 0x10;
    // 2^4 - 1
    frame._opcode = data & 0x0F;

    // 读取mask、payload_len
    let res = _read_frame(br, 1)?;
    let data = res[0];
    frame._mask = data & 0x80 == 0x80;
    frame._payload_len = (data & 0x7F) as u64;

    match frame._payload_len {
        126 => {
            let data = _read_frame(br, 2)?;
            frame._payload_len = ((data[0] & 0xFF) as u64) << 8 | (data[1] & 0xFF) as u64;
        }
        127 => {
            let mut data = _read_frame(br, 8)?;
            _reverse_data(&mut data);
            let data: [u8; 8] = data.try_into().unwrap();
            frame._payload_len = unsafe { std::mem::transmute(&data) };
        }
        _ => {}
    }

    if frame._mask {
        let data = _read_frame(br, 4)?;
        frame._mask_key = data.try_into().unwrap();
    }
    Ok(())
}

/// 数据反转，处理大小端序
fn _reverse_data(data: &mut Vec<u8>) {
    let mut _tmp = 0;
    let len = data.len();
    for i in 0..len / 2 {
        _tmp = data[i + 1];
        data[i + 1] = data[len - i - 1];
        data[len - i - 1] = _tmp;
    }
}

/// 解码
fn _unmask(data: &mut Vec<u8>, mask: &[u8; 4]) {
    for i in 0..data.len() {
        data[i] ^= mask[i % 4];
    }
}

/// 读取帧
fn _read_frame(br: &mut BufReader<&TcpStream>, len: usize) -> SResult<Vec<u8>> {
    let mut puf = vec![0; len];
    let res = br.read(&mut puf);
    if res.is_err() {
        return common::errs::to_err(res.unwrap_err());
    }
    Ok(puf)
}

impl Default for WSServer {
    fn default() -> Self {
        Self {
            _status: WSStatus::default(),
            _expire: now_drt() + DEFAULT_TIMEOUT,
            _timeout: DEFAULT_TIMEOUT,
            handler: _none_handler,
            tcp_svr: TcpServer::default(),
        }
    }
}

fn _none_handler(_data: &BytesMut) -> Option<Vec<u8>> {
    None
}

#[derive(Debug, Clone, Copy)]
pub enum WSStatus {
    Start,
    End,
    Handling,
}

impl Default for WSStatus {
    fn default() -> Self {
        Self::End
    }
}
