use crate::common::LoopStatus;
use std::net::{TcpListener, TcpStream};

#[derive(Clone)]
pub struct TcpServer<'a> {
    _addr: &'a str,
}

impl<'a> TcpServer<'a> {
    pub fn with_addr(&mut self, addr: &'a str) -> &mut Self {
        self._addr = addr;
        self
    }

    pub fn start<F>(&self, mut handler: F)
    where
        F: FnMut(&TcpStream) -> LoopStatus<bool>,
    {
        let listener = TcpListener::bind(self._addr).expect("bind failed");
        // bind success
        for res_stream in listener.incoming() {
            // new request...
            match res_stream {
                Ok(stream) => match handler(&stream) {
                    LoopStatus::Break => break,
                    LoopStatus::Continue => continue,
                    _ => {}
                },
                Err(e) => eprintln!("e={:?}", e),
            }
        }
    }
}

impl<'a> Default for TcpServer<'a> {
    fn default() -> Self {
        Self {
            _addr: "127.0.0.1:7878",
        }
    }
}

fn _none_handle(_stream: &TcpStream) -> LoopStatus<bool> {
    LoopStatus::Break
}
