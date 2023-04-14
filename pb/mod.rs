//! protobuf

#[allow(unused_imports)]
use common::{
    cm_log,
    errs::{self, SResult},
    trace,
};
use protobuf::Message;

/// pb协议版本
const PB_VERSION: &'static str = "0.0.1";

pub mod body;
pub mod client;
pub mod header;
pub mod req;
pub(crate) mod route;
pub mod rsp;
pub mod server;
/// 测试文件
mod test_proto;

pub fn to_pb<T: Message>(msg: &T) -> SResult<Vec<u8>> {
    match msg.write_to_bytes() {
        Ok(x) => Ok(x),
        Err(e) => errs::to_err(format!("{:?}", e)),
    }
}

#[test]
fn test_send_pb() {
    cm_log::log_init(common::LevelFilter::Trace);

    let foo = test_proto::msg::Foo {
        first_field: 23,
        second_field: String::from("sad"),
        fourth_field: true,
        fifth_field: protobuf::EnumOrUnknown::from(test_proto::msg::EnFoo::SECOND),
        ..Default::default()
    };
    let mut rsp = test_proto::msg::Foo::default();

    client::send("127.0.0.1:7881", "a.a", "function", &foo, &mut rsp);
    trace!("rsp={:?}", rsp);
}

#[test]
fn test_start_svr() {
    use crate::tcp::server::Server;

    cm_log::log_init(common::LevelFilter::Trace);

    route::add_route("a.a", "function", |_req, _rsp| {
        trace!("req={:?}", _req);
    });
    route::add_route("a.a", "function2", |_req, _rsp| {
        trace!("function2");
    });
    server::PbServer::default().start();
}
