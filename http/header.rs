use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

use common::{errs::SResult, strings, time};

use super::errs;

pub type HeaderType = BTreeMap<String, String>;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Header {
    _inner: HeaderType,
    // boundary
    _boundary: Option<String>,
}

impl Header {
    pub(crate) fn new(header: &mut HeaderType) -> Self {
        let mut inner = HeaderType::default();
        set_header(&mut inner, "date", &time::now_milli_str());
        set_header(
            &mut inner,
            "content-type",
            "application/x-www-form-urlencoded; charset=UTF-8",
        );
        set_header(&mut inner, "server", "default-j");
        inner.append(header);
        Self {
            _inner: inner,
            _boundary: None,
        }
    }

    pub fn headers(&self) -> &HeaderType {
        &self._inner
    }

    pub fn headers_mut(&mut self) -> &mut HeaderType {
        &mut self._inner
    }

    pub fn set_directly(&mut self, key: &str, val: &str) {
        self._inner.insert(key.to_string(), val.to_string());
    }

    pub fn set(&mut self, key: &str, val: &str) {
        let key = strings::extract_normal_lower_char(key);
        let mut val = val.trim().to_string();
        // 若key为content-type的form-data，需获取val的boundary
        if &key == "content-type" {
            let valc = val.clone();
            let val_deal = valc.split_once(";");
            if val_deal.is_some() {
                let valu = val_deal.unwrap();
                val = valu.0.to_string();
                let boundary = valu.1.trim();
                let bound_sp = boundary.split_once("=");
                if bound_sp.is_some() {
                    let bound = bound_sp.unwrap();
                    //插入boundary
                    self._boundary = Some(bound.1.to_string());
                }
            }
        }
        self._inner.insert(key, val);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self._inner
            .get(&common::strings::extract_normal_lower_char(key))
            .clone()
    }

    pub(crate) fn get_boundary(&self) -> Option<String> {
        self._boundary.clone()
    }

    pub(crate) fn parse_head<'a>(&mut self, head: &'a str) -> SResult<Vec<&'a str>> {
        let lines = head.split("\r\n").collect::<Vec<_>>();
        //第一行字符串
        let first = match lines.get(0) {
            Some(x) => *x,
            None => return errs::err_format(),
        };
        //第一行数据
        let first_line = first.split_ascii_whitespace().collect::<Vec<_>>();
        if first_line.len() < 3 {
            return errs::err_format();
        }
        for i in 1..lines.len() {
            self._parse_header(lines[i]);
        }
        Ok(first_line.clone())
    }

    pub fn get_content_type(&self) -> Option<String> {
        let ct = self.get("content-type");
        if ct.is_none() {
            return None;
        }
        Some(ct.unwrap().clone())
    }

    pub fn get_content_length(&self) -> Option<usize> {
        let content_len = self._inner.get("content-length");
        if content_len.is_none() {
            return None;
        }
        match content_len.unwrap().parse::<usize>() {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }

    //转换header
    fn _parse_header(&mut self, text: &str) {
        if text.trim().is_empty() {
            return;
        }
        let head = text.split_once(':');
        if head.is_none() {
            return;
        }
        let head = head.unwrap();
        self.set(head.0, head.1);
    }
}

pub(crate) fn set_header<'a>(
    header: &'a mut HeaderType,
    key: &str,
    val: &str,
) -> &'a mut HeaderType {
    let key = strings::extract_normal_lower_char(key);
    let val = val.trim().to_string();
    header.insert(key, val);
    header
}

//① req/rsp
pub(crate) fn read_head(br: &mut BufReader<&TcpStream>) -> String {
    let mut buf = String::default();
    loop {
        let mut puf = String::default();
        if let Ok(_len) = br.read_line(&mut puf) {
            // println!("puf={:?}", puf);
            if puf.is_empty() || puf == "\r\n" {
                break;
            }
            buf.push_str(&puf);
        } else {
            break;
        }
    }
    buf
}
