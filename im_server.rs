#![allow(unused)]
use crate::common::LoopStatus;
use std::{
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
    time::SystemTime,
};

pub(crate) fn server(addr: &str) {
    let listener = TcpListener::bind(addr).expect("bind failed");
    println!("bind success.");
    // let mut buf = String::new();
    for stream in listener.incoming() {
        println!("connect success...");
        // buf.clear();
        let begin = SystemTime::now();
        match stream {
            Ok(mut ts) => {
                println!("cost: {:?}", SystemTime::now().duration_since(begin));
                let mut buf = Vec::new();
                loop {
                    use crate::http::rw;
                    match rw::read_from_net(&ts, &mut buf) {
                        LoopStatus::Break => {
                            if let Ok(len) = ts.write(b"program exit") {
                                println!("break");
                            }
                            break;
                        }
                        LoopStatus::Continue => {
                            if let Ok(len) = ts.write(b"program continue") {
                                println!("continue");
                            }
                            continue;
                        }
                        LoopStatus::Normal((req, len)) => {
                            println!("buf: {:?}", req);
                            if req.len() >= 4 && req.starts_with("exit") {
                                //程序退出
                                if let Ok(len) = ts.write(b"program exit") {
                                    println!("program exit");
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
                println!("e:{:?}", e);
            }
        }
    }
    println!("server stop");
}
