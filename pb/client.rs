use std::io::{BufReader, BufWriter, Write};

use protobuf::Message;

use common::{error, trace};

use crate::pb::req::PbRequest;

use super::rsp::PbResponse;

pub fn send<T: Message>(
    addr: &str,
    server_name: &str,
    function: &str,
    req_data: &T,
    rsp_data: &mut T,
) {
    // 发送
    let stream = std::net::TcpStream::connect(addr).expect("connect failed");
    let mut bw = BufWriter::new(&stream);

    let mut req = PbRequest::default();
    req.set_server_name(server_name.to_string());
    req.set_function(function.to_string());

    req.set_pb_body(req_data).unwrap();

    match bw.write(&req.new_req()) {
        Ok(x) => trace!("write len={}", x),
        Err(e) => error!("write err, err={:?}", e),
    }
    bw.flush().unwrap();

    // 接收回包
    let mut br = BufReader::new(&stream);
    let rsp = PbResponse::new(&mut br);
    if rsp.is_err() {
        error!("rsp err={:?}", rsp.unwrap_err());
        return;
    }
    let rsp = rsp.unwrap();
    rsp.parse_pb(rsp_data).unwrap();
}
