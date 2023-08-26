use std::{collections::BTreeMap, io::BufReader, net::TcpStream};

use common::{
    error,
    errs::{self, SResult},
    strings::trim_and_lower,
    trace,
};
use protobuf::Message;

use crate::common::errs::err_format;

use super::header::PbHeader;

/// 数据格式(忽略空格):
/// 版本 空格 状态码\r\n
/// Content-Length:长度\r\n
/// header\r\n
/// header\r\n
/// ...
/// \r\n
/// data
#[derive(Debug)]
pub struct PbResponse {
    _version: String,
    _code: u32, // 同http状态码
    _header: BTreeMap<String, String>,
    _body: Vec<u8>,
}

impl PbHeader for PbResponse {
    fn get_header(&self, key: &str) -> Option<&String> {
        self._header.get(&trim_and_lower(key))
    }
}

impl PbResponse {
    pub fn new(br: &mut BufReader<&TcpStream>) -> SResult<Self> {
        let header = crate::http::header::read_header(br);
        trace!("rsp header={:?}", header);
        // header
        let mut rsp = PbResponse::default();
        let mut headers = header.split("\r\n").collect::<Vec<_>>();
        // 第一行
        if let Some(x) = headers.get(0) {
            let first_line = x.split_ascii_whitespace().collect::<Vec<_>>();
            if first_line.len() < 2 {
                error!("err format, first_line={:?}", first_line);
                return err_format();
            }
            rsp._version = first_line[0].to_string();
            rsp._code = first_line[1].parse().unwrap();
        } else {
            error!("err format, headers={:?}", headers);
            return err_format();
        };
        headers.remove(0);
        for s in headers {
            if let Some((k, v)) = s.split_once(":") {
                let (key, val) = (trim_and_lower(k), v.trim().to_string());
                rsp._header.insert(key, val);
            }
        }

        // read body
        let body = super::body::read_body(br, &rsp);
        rsp.set_body(body);
        Ok(rsp)
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self._body = body;
    }

    pub fn set_code(&mut self, code: u32) {
        self._code = code;
    }

    pub fn set_headers(&mut self, headers: BTreeMap<String, String>) {
        self._header = headers;
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self._header.insert(key, value);
    }

    pub fn new_rsp(&self) -> Vec<u8> {
        let mut res = Vec::default();

        res.extend_from_slice(&format!("{} {}\r\n", super::PB_VERSION, self._code).as_bytes());
        for head in &self._header {
            res.extend_from_slice(format!("{}:{}\r\n", head.0, head.1).as_bytes());
        }
        res.extend_from_slice(&format!("Content-Length:{}\r\n\r\n", &self._body.len()).as_bytes());

        res.extend_from_slice(&self._body);
        res
    }

    pub fn set_pb_body<T: Message>(&mut self, msg: &T) -> SResult<()> {
        self._body = super::to_pb(msg)?;
        Ok(())
    }

    pub fn parse_pb<T: Message>(&self, msg: &mut T) -> SResult<()> {
        match msg.merge_from_bytes(&self._body) {
            Ok(_) => Ok(()),
            Err(e) => errs::to_err(format!("{:?}", e)),
        }
    }
}

impl Default for PbResponse {
    fn default() -> Self {
        Self {
            _version: super::PB_VERSION.to_string(),
            _code: 500,
            _header: BTreeMap::default(),
            _body: Vec::default(),
        }
    }
}
