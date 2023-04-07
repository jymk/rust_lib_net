//! websocket

#[allow(unused_imports)]
use common::debug;

// #![allow(unused)]
pub mod frame;
pub mod server;

/// websocket所需魔法数
const MAGIC: &'static str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

#[test]
fn test() {
    use crate::tcp::server::*;
    server::WSServer::default()
        .with_handler(|data| {
            debug!("data={:?}", data);
            None
        })
        .start();
    // let n = format!("{:0>4b}", 256);
    // debug!("{:?}", u8::from_str_radix(&n, 2));
    // let a = 655346;
    // let b = a & 0xFF;
    // let c = a >> 8;
    // debug!("{:?}", (b, c));
}
