#![allow(unused)]
use common::{debug, status::LoopStatus};
use std::{
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
    time::SystemTime,
};

pub(crate) fn server(addr: &str) {
    let listener = TcpListener::bind(addr).expect("bind failed");
    debug!("bind success.");
    // let mut buf = String::new();
    use super::http::rw;
    for stream in listener.incoming() {
        debug!("connect success...");
        // buf.clear();
        let begin = SystemTime::now();
        match stream {
            Ok(mut ts) => {
                debug!("cost: {:?}", SystemTime::now().duration_since(begin));
                let mut buf = Vec::new();
                loop {
                    match rw::read_from_net(&ts, &mut buf) {
                        LoopStatus::Break => {
                            if let Ok(len) = ts.write(b"program exit") {
                                debug!("break");
                            }
                            break;
                        }
                        LoopStatus::Continue => {
                            if let Ok(len) = ts.write(b"program continue") {
                                debug!("continue");
                            }
                            continue;
                        }
                        LoopStatus::Normal((req, len)) => {
                            debug!("buf: {:?}", req);
                            if req.len() >= 4 && req.starts_with("exit") {
                                //程序退出
                                if let Ok(len) = ts.write(b"program exit") {
                                    debug!("program exit");
                                }
                                break;
                            } else {
                                if let Ok(len) = ts.write(req.as_bytes()) {}
                            }
                        }
                    }
                }
            }
            Err(e) => {
                debug!("e:{:?}", e);
            }
        }
    }
    debug!("server stop");
}
