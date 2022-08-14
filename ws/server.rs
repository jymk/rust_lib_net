use std::{
    io::{BufReader, BufWriter, Read, Write},
    net::TcpStream,
};

use bytes::BufMut;
use sha1::{Digest, Sha1};

use super::frame::_Frame;
use crate::http::{req::*, rsp::*};
use crate::LoopStatus;
use crate::{
    http::server::{back_with_header, write_msg},
    SResult,
};

#[derive(Debug, Clone, Copy)]
pub struct WSServer<'a> {
    _addr: &'a str,
    _path: &'a str,
    _status: WSStatus,
}

impl<'a> WSServer<'a> {
    pub fn with_addr(&mut self, addr: &'a str) -> &mut Self {
        self._addr = addr;
        self
    }
    pub fn with_path(&mut self, path: &'a str) -> &mut Self {
        self._path = path;
        self
    }

    pub fn start(&mut self) {
        crate::tcp::server::TcpServer::default()
            .with_addr(self._addr)
            .start(|stream| {
                self._status = WSStatus::Start;
                loop {
                    match self._status {
                        WSStatus::Start => self.on_start(stream),
                        WSStatus::End => break,
                        WSStatus::Handling => self.on_msg(stream),
                    }
                }
                LoopStatus::Break
            });
    }

    fn on_msg(&mut self, stream: &TcpStream) {
        let mut br = BufReader::new(stream);

        let mut frame = _Frame::default();
        loop {
            let res = _read_head(&mut br, &mut frame);
            if res.is_err() {
                eprintln!("_read_head: err={:?}", res.unwrap_err());
                self._status = WSStatus::End;
                return;
            }
            if frame._opcode == 8 {
                break;
            }
            let res = _read_data(&mut br, &mut frame);
            if res.is_err() {
                eprintln!("_read_data: err={:?}", res.unwrap_err());
                return;
            }
            println!("frame={:?}", frame);
            if frame._fin {
                break;
            }
        }
        println!("收到消息");
        // 向前端写数据应该也需要按照frame格式来
        // write_msg(stream, );
    }

    fn on_start(&mut self, stream: &TcpStream) {
        let mut br = BufReader::new(stream);
        let buf = crate::http::header::read_head(&mut br);
        // println!("\nreq={:?}", buf);
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

        println!("header={:?}", req.get_header());
        let mut rsp = HttpResponse::default();
        // 计算
        let wskey = req.get_header().get("Sec-WebSocket-Key");
        let wsversion = req.get_header().get("Sec-WebSocket-Version");
        if let Some(key) = &wskey {
            let mut hasher = Sha1::new();
            hasher.update(key.to_string() + super::MAGIC);
            let res = hasher.finalize();
            rsp.set_header("Sec-WebSocket-Accept", &crate::base64_encode(&res[..]));
            rsp.set_header("Sec-WebSocket-Version", wsversion.unwrap());
            rsp.set_header("Connection", "Upgrade");
            rsp.set_header("Upgrade", "websocket");
        }

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

fn _read_data(br: &mut BufReader<&TcpStream>, frame: &mut _Frame) -> SResult<()> {
    let mut res = _read_frame(br, frame._payload_len as usize)?;
    if frame._mask {
        _unmask(&mut res, &frame._mask_key);
    }
    frame._data.put_slice(&res);
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
    let mut tmp = 0;
    let len = data.len();
    for i in 0..len / 2 {
        tmp = data[i + 1];
        data[i + 1] = data[len - i - 1];
        data[len - i - 1] = tmp;
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
        return crate::sresult_from_err(res.unwrap_err());
    }
    Ok(puf)
}

impl<'a> Default for WSServer<'a> {
    fn default() -> Self {
        Self {
            _addr: "127.0.0.1:7880",
            _path: "/ws",
            _status: WSStatus::default(),
        }
    }
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
