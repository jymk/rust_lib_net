//! websocket

#[allow(unused_imports)]
use common::{trace, cm_log};

// #![allow(unused)]
pub mod frame;
pub mod server;

/// websocket所需魔法数
const MAGIC: &'static str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

#[test]
fn test() {
	cm_log::log_init(common::LevelFilter::Trace);
    use crate::tcp::server::*;
    let mut svr = server::WSServer::default();
    svr.with_handler(|data| {
            trace!("data={:?}", data);
            None
        });
	svr.start();
    // let n = format!("{:0>4b}", 256);
    // trace!("{:?}", u8::from_str_radix(&n, 2));
    // let a = 655346;
    // let b = a & 0xFF;
    // let c = a >> 8;
    // trace!("{:?}", (b, c));
}
