use common::{error, status::LoopStatus};
use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct TcpServer {
    _addr: &'static str,
    // 线程数量，默认1
    _thread_num: usize,
}

impl TcpServer {
    pub fn with_addr(&mut self, addr: &'static str) -> &mut Self {
        self._addr = addr;
        self
    }

    pub fn with_thread_num(&mut self, thread_num: usize) -> &mut Self {
        self._thread_num = thread_num;
        self
    }

    pub fn start<F>(&self, handler: F)
    where
        F: FnMut(&TcpStream) -> LoopStatus<bool> + std::marker::Send + 'static,
    {
        let listener = TcpListener::bind(self._addr).expect("bind failed");
        let stop_flag = Arc::new(Mutex::new(false));
        let pool = super::worker::ThreadPool::new(self._thread_num);

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
            _addr: "127.0.0.1:80",
            _thread_num: 1,
        }
    }
}

fn _none_handle(_stream: &TcpStream) -> LoopStatus<bool> {
    LoopStatus::Break
}

pub trait Server: 'static {
    /// 启动服务
    fn start(&'static mut self);
}

#[test]
fn test_tcp_server() {
    TcpServer::default()
        .with_addr("127.0.0.1:7878")
        .start(|_stream| LoopStatus::Break)
}
