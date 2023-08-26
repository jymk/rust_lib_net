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

/// 注: URI包含服务名，其格式为/xxxxxx/xxx
/// 数据格式(忽略空格):  
/// URI 空格 协议版本\r\n
/// Content-Length:长度\r\n
/// header\r\n
/// header\r\n
/// ...\r\n
/// \r\n
/// data
#[derive(Debug)]
pub struct PbRequest {
    _server_name: String,
    _function: String,
    _version: String,
    _header: BTreeMap<String, String>,
    _body: Vec<u8>,
}

impl PbHeader for PbRequest {
    fn get_header(&self, key: &str) -> Option<&String> {
        self._header.get(&trim_and_lower(key))
    }
}

impl PbRequest {
    pub(crate) fn new(br: &mut BufReader<&TcpStream>) -> SResult<Self> {
        let header = crate::http::header::read_header(br);
        trace!("req header={}", header);
        let mut req = PbRequest::default();
        let mut heads = header.split("\r\n").collect::<Vec<_>>();

        // 第一行
        if let Some(x) = heads.get(0) {
            let first_line = x.trim().split_ascii_whitespace().collect::<Vec<_>>();
            if first_line.len() < 2 {
                error!("err format, first_line={:?}", first_line);
                return err_format();
            }
            //TODO 验证版本
            req._version = first_line[1].to_string();
            // 分离server_name和function
            let uri = first_line[0].split("/").collect::<Vec<_>>();
            if uri.len() < 3 {
                error!("err uri, uri={:?}", uri);
                return err_format();
            }
            req._server_name = uri[1].to_string();
            req._function = uri[2].to_string();
        } else {
            error!("err format, heads={:?}", heads);
            return err_format();
        }
        // headers
        heads.remove(0);
        for s in heads {
            if let Some((k, v)) = s.split_once(":") {
                let (key, val) = (trim_and_lower(k), v.trim().to_string());
                req._header.insert(key, val);
            }
        }
        trace!("header={:?}", req._header);

        let body = super::body::read_body(br, &req);
        req.set_body(body);
        Ok(req)
    }

    pub fn new_req(&self) -> Vec<u8> {
        let mut rsp = format!(
            "/{}/{} {}\r\n",
            self._server_name,
            self._function,
            super::PB_VERSION
        )
        .as_bytes()
        .to_vec();
        for head in &self._header {
            rsp.extend_from_slice(format!("{}:{}\r\n", head.0, head.1).as_bytes());
        }
        rsp.extend_from_slice(format!("Content-Length:{}\r\n\r\n", &self._body.len()).as_bytes());
        rsp.extend_from_slice(&self._body);
        rsp
    }

    pub fn function(&self) -> &String {
        &self._function
    }

    pub fn server_name(&self) -> &String {
        &self._server_name
    }

    pub fn set_function(&mut self, function: String) {
        self._function = function;
    }

    pub fn set_server_name(&mut self, server_name: String) {
        self._server_name = server_name;
    }

    pub fn get_headers(&self) -> &BTreeMap<String, String> {
        &self._header
    }

    pub fn set_body(&mut self, data: Vec<u8>) {
        self._body = data;
    }

    pub fn set_pb_body<T: Message>(&mut self, msg: &T) -> SResult<()> {
        self._body = super::to_pb(msg)?;
        Ok(())
    }

    pub fn set_headers(&mut self, headers: BTreeMap<String, String>) {
        self._header = headers;
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self._header.insert(key, value);
    }

    pub fn parse_pb<T: Message>(&self, msg: &mut T) -> SResult<()> {
        match msg.merge_from_bytes(&self._body) {
            Ok(_) => Ok(()),
            Err(e) => errs::to_err(format!("{:?}", e)),
        }
    }
}

impl Default for PbRequest {
    fn default() -> Self {
        Self {
            _server_name: Default::default(),
            _function: Default::default(),
            _version: super::PB_VERSION.to_string(),
            _header: Default::default(),
            _body: Default::default(),
        }
    }
}
