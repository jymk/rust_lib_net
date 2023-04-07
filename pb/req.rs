use std::{collections::BTreeMap, io::BufReader, net::TcpStream};

use common::{
    debug, error,
    errs::{self, SResult},
    strings::trim_and_lower,
};
use protobuf::Message;

use crate::common::errs::err_format;

/// 数据格式(忽略空格):  
/// URI 空格 协议版本\r\n
/// header\r\n
/// header\r\n
/// ...\r\n
/// \r\n
/// data
#[derive(Debug, Default)]
pub struct PbRequest {
    _uri: String,
    _version: String,
    _header: BTreeMap<String, String>,
    _body: Vec<u8>,
}

impl PbRequest {
    pub(crate) fn new(br: &mut BufReader<&TcpStream>) -> SResult<Self> {
        let mut req = PbRequest::default();

        let head = crate::http::header::read_head(br);
        let mut heads = head.split("\r\n").collect::<Vec<_>>();

        // 第一行
        if let Some(x) = heads.get(0) {
            let first_line = x.trim().split_ascii_whitespace().collect::<Vec<_>>();
            if first_line.len() < 2 {
                error!("err format, first_line={:?}", first_line);
                return err_format();
            }
            req._uri = first_line[0].to_string();
            req._version = first_line[1].to_string();
        } else {
            error!("err format, heads={:?}", heads);
            return err_format();
        }
        // headers
        heads.remove(0);
        for s in heads {
            if let Some((k, v)) = s.split_once(":") {
                let (key, val) = (trim_and_lower(k), v.trim().to_string());
                debug!("key='{}', val='{}'", key, val);
                req._header.insert(key, val);
            }
        }
        debug!("header={:?}", req._header);
        Ok(req)
    }

    pub fn get_header(&self, key: &str) -> Option<&String> {
        self._header.get(&trim_and_lower(key))
    }

    pub fn set_body(&mut self, data: Vec<u8>) {
        self._body = data;
    }

    pub fn parse_pb<T: Message>(&self, msg: &mut T) -> SResult<()> {
        match msg.merge_from_bytes(&self._body) {
            Ok(_) => Ok(()),
            Err(e) => errs::to_err(format!("{:?}", e)),
        }
    }
}
