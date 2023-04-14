mod common;
mod http;
mod im_client;
mod im_server;
pub mod pb;
pub mod route;
mod tcp;
pub mod ws;

#[allow(unused_imports)]
use ::common::{cm_log, trace};

pub use bytes;
pub use http::{
    header::{Header as HttpHeader, HeaderType},
    req::HttpRequest,
    route::{add_get_route, add_post_route, print_routes},
    rsp::*,
    server::HttpServer,
    Body as HttpBody,
};

pub use ws::server::WSServer;

#[test]
fn test() {
    cm_log::log_init(::common::LevelFilter::Debug);
    trace!("Hello, world!");
    let args = std::env::args().collect::<Vec<_>>();
    trace!("args={:?}, arglen={}", args, args.len());
    if args.len() < 2 {
        trace!("miss param");
        panic!("miss param");
    }
    if &args[1] == "s" {
        im_server::server(&"0.0.0.0:7879");
    }
    if &args[1] == "c" {
        im_client::client(&"localhost:7879");
    }
}
