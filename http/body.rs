#[allow(unused_imports)]
use common::{debug, error, status::LoopStatus};

use super::{errs, header::Header, req::HttpMethod};
// use serde_json::Value;
use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use common::errs::SResult;

use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Body {
    _inner: BytesMut,
}

impl Body {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_body<T: Into<BytesMut>>(&mut self, body: T) {
        self._inner = body.into();
    }

    pub fn len(&self) -> usize {
        self._inner.len()
    }

    pub fn get_u8s(&self) -> Vec<u8> {
        self._inner.to_vec()
    }

    pub fn get_body(&self) -> &BytesMut {
        &self._inner
    }

    //根据头信息读取body
    pub(crate) fn obtain_body(
        &mut self,
        br: &mut BufReader<&TcpStream>,
        header: &Header,
        method: HttpMethod,
    ) -> SResult<()> {
        //若有Transfer-Encoding，且其为chunked
        let te = header.get("transfer-encoding");
        if te.is_some() {
            // 若为file，目前只能读取一个file
            let te = te.unwrap();
            if te == "chunked" {
                // let mut v = Vec::<Value>::default();
                //扩展长度\r\n扩展\r\n数据长度\r\n数据\r\n\r\n扩展长度\r\n扩展\r\n数据长度\r\n数据\r\n
                //数字0\r\n\r\n
                loop {
                    let extension = _read_chunked(br);
                    let ext = match extension {
                        LoopStatus::Break => {
                            break;
                        }
                        LoopStatus::Continue => continue,
                        LoopStatus::Normal(x) => x.clone(),
                    };

                    self._inner.put_slice(&ext);
                }
                return Ok(());
            }
        }

        //若无Transfer-Encoding则读取content_length
        let mut content_length = header.get_content_length();
        // trace!("content_length={:?}", content_length);
        if content_length.is_none() {
            //no content
            if method == HttpMethod::POST {
                return common::errs::to_err("Content-Length is none");
            } else {
                content_length = Some(0);
            }
        }
        let content_length = content_length.unwrap();
        if content_length == 0 {
            return Ok(());
        }
        let mut buf = vec![0; content_length];
        //读取body
        match br.read(&mut buf) {
            Ok(_len) => {
                buf.truncate(content_length);
                self._inner.put_slice(&buf);
                return Ok(());
            }
            Err(e) => {
                error!("e={:?}", e);
                return errs::err_server_internal();
            }
        }
    }

    /// body: 分为文件类型的存储和非文件类型的存储
    /// 文件类型应该只有form
    /// 非文件类型又分为form/...
    #[allow(unused)]
    pub(crate) fn analyze_body(&self, header: &Header) -> BTreeMap<String, String> {
        //非文件
        let ct = header.get_content_type();
        // trace!("ct={:?}", ct);
        if ct.is_none() {
            return BTreeMap::default();
        }
        let content_type = ct.unwrap();
        match content_type.as_str() {
            // "multipart/form-data" => self.analyze_form(header),
            "application/x-www-form-urlencoded" => analyze_param(
                // 此种content_type必定有body
                &*String::from_utf8_lossy(&self._inner.to_vec()),
            ),
            // application/javascript、text/plain、text/html、application/xml
            // application/json需json解析，也不处理
            _ => BTreeMap::default(),
        }
    }

    pub(crate) fn analyze_param(&self) -> BTreeMap<String, String> {
        analyze_param(&*String::from_utf8_lossy(&self._inner.to_vec()))
    }

    /// 若为二进制文件不能转为string
    /// form-data 非文件非chunked
    pub(crate) fn analyze_form(&self, header: &Header) -> BTreeMap<String, Vec<u8>> {
        let mut params = BTreeMap::default();
        let boundary = header.get_boundary();
        // trace!("boundary={:?}", boundary);
        if boundary.is_none() {
            error!("no boundary");
            //无boundary直接返回
            return params;
        }
        let boundary = boundary.clone().unwrap();

        // 转码body
        // 解析，以boundary分隔
        let mut sep_boundary = String::from("--");
        sep_boundary.push_str(&boundary);

        let end_boundary = sep_boundary.clone() + "--\r\n";
        sep_boundary.push_str("\r\n");

        let mut data_str = self._inner.to_vec();
        if &data_str[..sep_boundary.len()] == sep_boundary.as_bytes() {
            data_str = data_str[sep_boundary.len()..].to_vec();
        }
        if &data_str[data_str.len() - end_boundary.len()..] == end_boundary.as_bytes() {
            data_str = data_str[..data_str.len() - end_boundary.len()].to_vec();
        }
        let mut data_str_tmp = data_str.clone();
        let mut data_vec = Vec::default();
        while data_str_tmp.len() < sep_boundary.len() {
            let data_id = data_str_tmp
                .windows(sep_boundary.len())
                .position(|s| s == sep_boundary.as_bytes());
            if data_id.is_none() {
                data_vec.push(data_str_tmp.clone());
                break;
            }
            let data_id = data_id.unwrap();
            data_vec.push(data_str_tmp[data_id + sep_boundary.len()..sep_boundary.len()].to_vec());
            data_str_tmp = data_str_tmp[sep_boundary.len()..].to_vec();
        }

        //处理参数
        for data in data_vec {
            _deal_form(&mut params, &data);
        }

        params
    }
}

/// 处理form头
/// Content-Disposition: form-data; name=\"vk\"; filename=\"vk_swiftshader_icd.json\"\r\n\r\n3\r\n
fn _deal_form(params: &mut BTreeMap<String, Vec<u8>>, s: &[u8]) {
    let ext = s
        .iter()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| *c)
        .collect::<Vec<_>>(); //.split_once("\r\n\r\n");
    let id = ext.windows(4).position(|s| s == b"\r\n\r\n");
    if id.is_none() {
        error!("form data error");
        return;
    }
    let id = id.unwrap();
    let one = ext[..id].to_vec();
    let two = ext[id + 4..].to_vec();
    // if ext.is_none() {
    //     return;
    // }
    // let data = ext.unwrap();
    let datas = one.splitn(2, |&c| c == b';').collect::<Vec<_>>();
    if datas.len() != 2 {
        error!("form data error2");
        return;
    }
    // let datas = data.0.split(";").map(|x| x.trim()).collect::<Vec<_>>();
    for d in datas {
        let sep = if d.starts_with(b"Content-Disposition") {
            ':'
        } else {
            '='
        };
        let tmp = d.splitn(2, |&c| c == sep as u8).collect::<Vec<_>>();
        if tmp.len() != 2 {
            continue;
        }
        if tmp[0] == b"name" {
            let (mut start_index, mut end_index) = (0, 0);
            for (i, &v) in tmp[1].iter().enumerate() {
                if v != b'"' {
                    start_index = i;
                    break;
                }
            }
            let mut i = tmp[1].len() - 1;
            while i >= 0 {
                if tmp[1][i] != b'"' {
                    end_index = i;
                    break;
                }
                i -= 1;
            }
            let newtmp = &*String::from_utf8_lossy(&tmp[1][start_index..=end_index]);
            params.insert(newtmp.to_string(), two);
            return;
        }
    }
}

/// 读取chunked数据
fn _read_chunked(br: &mut BufReader<&TcpStream>) -> LoopStatus<Vec<u8>> {
    let mut res = LoopStatus::default();
    loop {
        //读取下一行数据长度
        let mut puf = Vec::default();
        let len = br.read_until(b'\n', &mut puf);
        // trace!("linelen={:?}", len);
        if len.is_err() {
            return res;
        }

        // trace!("puflen={}", puf.len());
        // common::u8s_to_chars(&puf);
        let num = String::from_utf8(puf);
        if num.is_err() {
            // trace!("puf={:?}, err={:?}", puf, num);
            return res;
        }
        let num = num.unwrap();
        let num = num.trim().trim_matches('\0');

        // trace!("num={}:[{}]", num.is_empty(), num);
        //如果为空白行，继续下一行，除非行数据长度为0
        if num.is_empty() {
            continue;
        }
        //解析数据长度
        let len = usize::from_str_radix(num, 16);
        if len.is_err() {
            return res;
        }

        // trace!("len={:?}", len);

        //根据数据长度读取数据
        let size = len.unwrap();
        let mut buf = vec![0; size];
        let len = br.read(&mut buf);
        if len.is_err() {
            return res;
        }
        buf.truncate(size);
        //去除无用\r\n
        // if size == 2 && buf == [13, 10] {
        //     continue;
        // }
        // trace!("size={:?}", size);

        //若读完最后一行，跳出循环
        if size == 0 {
            return res;
        }
        res = LoopStatus::Normal(buf);
        break;
    }
    res
}

/// params或x-www-form-urlencoded解析
pub fn analyze_param(s: &str) -> BTreeMap<String, String> {
    let mut params = BTreeMap::default();
    if s.is_empty() {
        return params;
    }
    let all = s.split("&").collect::<Vec<_>>();
    for single in all {
        let data = single.split_once("=");
        if data.is_none() {
            continue;
        }
        let data = data.unwrap();
        params.insert(data.0.to_string(), data.1.to_string());
    }
    params
}

impl ToString for Body {
    fn to_string(&self) -> String {
        String::from(&*String::from_utf8_lossy(&self._inner.to_vec()))
    }
}
