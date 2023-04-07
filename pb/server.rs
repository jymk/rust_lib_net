use std::io::{BufReader, BufWriter, Read, Write};

#[allow(unused_imports)]
use common::{cm_log, debug, error, status::LoopStatus};

use crate::tcp::server::*;

#[derive(Debug)]
pub struct PbServer<'a> {
    _addr: &'a str,
}

impl<'a> PbServer<'a> {}

impl<'a> Server<'a> for PbServer<'a> {
    fn with_addr(&mut self, addr: &'a str) -> &mut Self {
        self._addr = addr;
        self
    }

    fn start(&mut self) {
        TcpServer::default().with_addr(self._addr).start(|stream| {
            let mut br = BufReader::new(stream);

            let req_res = super::req::PbRequest::new(&mut br);
            if req_res.is_err() {
                error!("req_err={:?}", req_res.unwrap_err());
                return LoopStatus::Break;
            }
            let mut req = req_res.unwrap();

            let content_len = req.get_header("Content-Length");
            let content_len = if content_len.is_none() {
                0
            } else {
                let len = content_len.unwrap().parse::<usize>();
                if len.is_err() {
                    error!(
                        "content_len is err, content_len={:?}, err={:?}",
                        content_len,
                        len.unwrap_err()
                    );
                    0
                } else {
                    len.unwrap()
                }
            };
            debug!("content_len={}", content_len);
            let mut buf = vec![0; content_len];
            match br.read(&mut buf) {
                Ok(x) => debug!("read len={}, buf={:?}", x, String::from_utf8(buf.clone())),
                Err(e) => {
                    error!("read data err, err={:?}", e);
                    return LoopStatus::Continue;
                }
            }
            req.set_body(buf.clone());

            // 回包
            let mut bw = BufWriter::new(stream);
            let rsp = super::rsp::PbResponse::new_rsp(buf);
            match bw.write(&rsp) {
                Ok(x) => debug!("write len={}", x),
                Err(e) => error!("write err, err={:?}", e),
            }

            LoopStatus::Continue
        });
    }
}

impl<'a> Default for PbServer<'a> {
    fn default() -> Self {
        Self {
            _addr: "127.0.0.1:7881",
        }
    }
}

#[test]
fn test_pbsvr() {
    cm_log::log_init(common::LevelFilter::Debug);
    PbServer::default().start();
}

#[test]
fn test_send_pb() {
    cm_log::log_init(common::LevelFilter::Debug);

    let stream = std::net::TcpStream::connect("127.0.0.1:7881").expect("connect failed");
    let mut bw = BufWriter::new(stream);

    let text = b"/senda v1.1\r\nContent-Length: 2\r\n\r\ner";
    let rsp = super::rsp::PbResponse::new_rsp(text.to_vec());
    match bw.write(&rsp) {
        Ok(x) => debug!("write len={}", x),
        Err(e) => error!("write err, err={:?}", e),
    }
}
