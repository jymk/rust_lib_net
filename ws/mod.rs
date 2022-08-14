#![allow(unused)]
pub mod frame;
pub mod server;

/// websocket所需魔法数
const MAGIC: &'static str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

#[test]
fn test() {
    server::WSServer::default().start();
}
