#![allow(unused)]
use std::{
    cell::RefCell,
    io::{stdin, BufRead, BufReader, Read, Write},
    net::TcpStream,
    rc::Rc,
    time::SystemTime,
};

use super::http::rw::{self, write_from_cmd};
use crate::common::LoopStatus;

pub(crate) fn client(addr: &str) -> bool {
    let begin = SystemTime::now();
    let mut stream = TcpStream::connect(&addr).expect("connect failed");
    println!(
        "Successfully connected to server in addr is {},  cost: {:?}",
        addr,
        SystemTime::now().duration_since(begin)
    );
    let mut wbuf = String::new();

    loop {
        write_from_cmd(&stream, &mut wbuf);
        let mut rbuf = Vec::<u8>::new();
        match rw::read_from_net(&stream, &mut rbuf) {
            LoopStatus::Break => {
                break;
            }
            LoopStatus::Continue => {
                continue;
            }
            LoopStatus::Normal((s, _)) => {
                if s == "program exit" {
                    break;
                }
                println!("r_result: {}", s);
            }
        }
    }
    println!("client stop");

    return true;
}
