use std::{
    fmt::Debug,
    io::{BufReader, BufWriter, Write},
    net::TcpStream,
};

#[allow(unused_imports)]
use common::{cm_log, error, status::LoopStatus, trace};

use crate::{
    pb::{req::PbRequest, rsp::PbResponse},
    tcp::server::*,
};

type BeforeType = fn(&PbRequest, &mut PbResponse) -> bool;
type AfterType = fn(&PbRequest, &mut PbResponse);

#[derive(Clone)]
pub struct PbServer {
    pub tcp_svr: TcpServer,
    _before: BeforeType,
    _after: AfterType,
    _stop: bool,
}

impl PbServer {
    pub fn with_before(&mut self, before: BeforeType) -> &mut Self {
        self._before = before;
        self
    }

    pub fn with_after(&mut self, after: AfterType) -> &mut Self {
        self._after = after;
        self
    }

    pub fn stop(&mut self) {
        self._stop = true;
    }

    fn handle(&self, stream: &TcpStream) {
        let mut br = BufReader::new(stream);

        let req_res = PbRequest::new(&mut br);
        if req_res.is_err() {
            error!("req_err={:?}", req_res.unwrap_err());
            return;
        }
        let req = req_res.unwrap();

        let mut rsp = PbResponse::default();

        //方法前执行
        let check = (self._before)(&req, &mut rsp);
        if !check {
            (self._after)(&req, &mut rsp);
            rsp.set_code(401);
            _back(stream, &rsp);
            return;
        }

        let has_func = super::route::fun(req.server_name(), req.function());
        if has_func.is_none() {
            rsp.set_code(404);
            _back(stream, &rsp);
            return;
        }

        has_func.unwrap()(&req, &mut rsp);

        // 方法后执行
        (self._after)(&req, &mut rsp);

        rsp.set_code(200);
        _back(stream, &rsp);
    }
}

impl Server for PbServer {
    fn start(self) {
        let this = self.clone();
        self.tcp_svr.start(move |stream| {
            if this._stop {
                return LoopStatus::Break;
            }
            let result = std::panic::catch_unwind(|| {
                this.handle(&stream);
            });
            if result.is_err() {
                error!("servre handle err={:?}", result.unwrap_err());
            }
            LoopStatus::Continue
        });
    }
}

impl Default for PbServer {
    fn default() -> Self {
        Self {
            _before: _none_before,
            _after: _none_after,
            _stop: false,
            tcp_svr: TcpServer::default(),
        }
    }
}

impl Debug for PbServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PbServer")
            .field("_tcp_svr", &self.tcp_svr)
            .field("_before", &"[before_func]")
            .field("_after", &"[after_func]")
            .field("_stop", &self._stop)
            .finish()
    }
}

fn _none_before(_req: &PbRequest, _rsp: &mut PbResponse) -> bool {
    true
}

fn _none_after(_req: &PbRequest, _rsp: &mut PbResponse) {}

fn _back(stream: &TcpStream, rsp: &PbResponse) {
    // 回包
    let mut bw = BufWriter::new(stream);
    let rsp = rsp.new_rsp();
    match bw.write(&rsp) {
        Ok(x) => trace!("write len={}", x),
        Err(e) => error!("write err, err={:?}", e),
    }
    bw.flush().unwrap();
}
