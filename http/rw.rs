use std::{
    io::{stdin, BufRead, BufReader, Write},
    net::TcpStream,
};

use common::{debug, error, status::LoopStatus};

pub(crate) fn read_from_net<'a>(
    ts: &TcpStream,
    buf: &'a mut Vec<u8>,
) -> LoopStatus<(&'a str, usize)> {
    //这里若不clear，会使数据非utf8
    buf.clear();
    let mut reader = BufReader::new(ts);
    let rsize = match reader.read_until(b'\n', buf) {
        Ok(len) => len,
        Err(e) => {
            error!("read err: {:?}", e);
            return LoopStatus::Break;
        }
    };
    let req = match std::str::from_utf8(&buf[..rsize]) {
        Ok(x) => x,
        Err(e) => {
            error!("{}", format!("need utf-8 sequence, {:?}", e));
            return LoopStatus::Continue;
        }
    };
    LoopStatus::Normal((req, rsize))
}

pub(crate) fn write_from_cmd(stream: &TcpStream, buf: &mut String) -> usize {
    buf.clear();
    let size = match stdin().read_line(buf) {
        Ok(len) => len,
        Err(_) => return 0,
    };
    debug!("size:{}", size);
    let wsize = write_text(stream, &buf.as_bytes()[..size]);
    debug!("wsize:{}", wsize);
    wsize
}
pub(crate) fn write_text(mut stream: &TcpStream, buf: &[u8]) -> usize {
    let wsize = match stream.write(buf) {
        Ok(len) => len,
        Err(_) => return 0,
    };
    debug!("wsize:{}", wsize);
    wsize
}
