use common::{error, status::LoopStatus};
use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct TcpServer {
    pub addr: &'static str,
    // 线程数量，默认1，必须大于0
    pub thread_num: usize,
}

impl TcpServer {
    pub fn start<F>(&self, handler: F)
    where
        F: FnMut(&TcpStream) -> LoopStatus<bool> + std::marker::Send + 'static,
    {
        let listener = TcpListener::bind(self.addr).expect("bind failed");
        let stop_flag = Arc::new(Mutex::new(false));
        let pool = super::worker::ThreadPool::new(self.thread_num);

        let handler = Arc::new(Mutex::new(handler));

        // bind success
        for res_stream in listener.incoming() {
            // new request...
            if *stop_flag.lock().unwrap() {
                break;
            }
            let stop_flag_clone = Arc::clone(&stop_flag);
            let handler_clone = handler.clone();
            match res_stream {
                Ok(stream) => {
                    pool.execute(move || match handler_clone.lock().unwrap()(&stream) {
                        LoopStatus::Break => *stop_flag_clone.lock().unwrap() = true,
                        _ => {}
                    });
                }
                Err(e) => error!("e={:?}", e),
            }
        }
    }
}

impl Default for TcpServer {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:80",
            thread_num: 1,
        }
    }
}

fn _none_handle(_stream: &TcpStream) -> LoopStatus<bool> {
    LoopStatus::Break
}

pub trait Server: 'static + Clone {
    /// 启动服务
    fn start(self);
}

#[test]
fn test_tcp_server() {
    let mut svr = TcpServer::default();
    svr.addr = "localhost:80";
    svr.start(|_stream| LoopStatus::Break)
}
