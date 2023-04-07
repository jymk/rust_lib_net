use std::collections::BTreeMap;

use common::errs::{self, SResult};
use protobuf::Message;

/// 数据格式(忽略空格):
/// 版本 空格 状态码\r\n
/// header\r\n
/// header\r\n
/// ...
/// \r\n
/// data
#[derive(Debug, Default)]
pub struct PbResponse {
    _version: String,
    _code: u32, // 同http状态码
    _header: BTreeMap<String, String>,
    _body: Vec<u8>,
}

impl PbResponse {
    pub fn new_rsp(body: Vec<u8>) -> Vec<u8> {
        let mut res = Vec::default();

        let mut header = String::default();
        header.push_str(super::PB_VERSION);
        header.push_str("\r\n");

        header.push_str(&("Content-Length:".to_string() + &body.len().to_string()));
        header.push_str("\r\n");
        header.push_str("\r\n");

        res.extend_from_slice(header.as_bytes());
        res.extend_from_slice(&body);
        res
    }

    pub fn to_pb<T: Message>(&mut self, msg: &T) -> SResult<()> {
        self._body = to_pb(msg)?;
        Ok(())
    }
}

pub fn to_pb<T: Message>(msg: &T) -> SResult<Vec<u8>> {
    match msg.write_to_bytes() {
        Ok(x) => Ok(x),
        Err(e) => errs::to_err(format!("{:?}", e)),
    }
}
