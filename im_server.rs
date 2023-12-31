#![allow(unused)]
use common::{status::LoopStatus, trace};
use std::{
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
    time::SystemTime,
};

pub(crate) fn server(addr: &str) {
    let listener = TcpListener::bind(addr).expect("bind failed");
    trace!("bind success.");
    // let mut buf = String::new();
    use super::http::rw;
    for stream in listener.incoming() {
        trace!("connect success...");
        // buf.clear();
        let begin = SystemTime::now();
        match stream {
            Ok(mut ts) => {
                trace!("cost: {:?}", SystemTime::now().duration_since(begin));
                let mut buf = Vec::new();
                loop {
                    match rw::read_from_net(&ts, &mut buf) {
                        LoopStatus::Break => {
                            if let Ok(len) = ts.write(b"program exit") {
                                trace!("break");
                            }
                            break;
                        }
                        LoopStatus::Continue => {
                            if let Ok(len) = ts.write(b"program continue") {
                                trace!("continue");
                            }
                            continue;
                        }
                        LoopStatus::Normal((req, len)) => {
                            trace!("buf: {:?}", req);
                            if req.len() >= 4 && req.starts_with("exit") {
                                //程序退出
                                if let Ok(len) = ts.write(b"program exit") {
                                    trace!("program exit");
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
                trace!("e:{:?}", e);
            }
        }
    }
    trace!("server stop");
}
