mod common;
mod http;
mod im_client;
mod im_server;
mod tcp;
mod ws;

pub use bytes::self;
pub use common::*;
pub use http::{
    header::{Header as HttpHeader, HeaderType},
    req::HttpRequest,
    route::{add_get_route, add_post_route, print_routes},
    rsp::*,
    server::Server as HttpServer,
    Body as HttpBody,
};

pub use ws::server::WSServer;

fn _main() {
    // println!("Hello, world!");
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        panic!("miss param");
    }
    if &args[1] == "s" {
        im_server::server(&"0.0.0.0:7879");
    }
    if &args[1] == "c" {
        im_client::client(&"localhost:7879");
    }
}
