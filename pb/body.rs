use std::{
    io::{BufReader, Read},
    net::TcpStream,
};

use common::{error, trace};

use super::header::PbHeader;

pub(crate) fn read_body<T: PbHeader>(br: &mut BufReader<&TcpStream>, obj: &T) -> Vec<u8> {
    // read body
    let content_len = obj.get_header("Content-Length");
    let content_len = if content_len.is_none() {
        0
    } else {
        let len = content_len.unwrap().parse::<usize>();
        if len.is_err() {
            error!(
                "content_len is err, content_len={:?}, err={:?}",
                content_len,
                len.unwrap_err()
            );
            0
        } else {
            len.unwrap()
        }
    };
    trace!("content_len={}", content_len);

    let mut buf = vec![0; content_len];
    match br.read(&mut buf) {
        Ok(x) => trace!("read len={}, buf={:?}", x, String::from_utf8(buf.clone())),
        Err(e) => {
            error!("read data err, err={:?}", e);
            return buf;
        }
    }
    buf
}
