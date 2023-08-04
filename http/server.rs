use std::{
    collections::BTreeMap,
    fmt::Debug,
    io::{BufReader, BufWriter, Write},
    net::TcpStream,
};

use crate::{http::http_handler, HttpRequest, HttpResponse, StatusCode};
#[allow(unused_imports)]
use common::cm_log;
#[allow(unused_imports)]
use common::{error, status::LoopStatus, trace};

use super::{
    super::tcp::server::*,
    header::{self, HeaderType},
    route,
};

pub type BeforeType = fn(&HttpRequest, &mut HttpResponse) -> bool;
pub type AfterType = fn(&HttpRequest, &mut HttpResponse);

#[derive(Clone)]
pub struct HttpServer {
    // 绑定地址(包括端口)
    pub tcp_svr: TcpServer,
    pub header: HeaderType,
    // 方法前执行
    pub before: BeforeType,
    // 方法后执行
    pub after: AfterType,
    _stop: bool,
}

impl HttpServer {
    /// 可指定回包状态码服务启动
    pub fn start_base(&self, suc_code: StatusCode) {
        let this = self.clone();
        self.tcp_svr.start(move |stream| {
            if this._stop {
                return LoopStatus::Break;
            }
            let result = std::panic::catch_unwind(|| {
                handler_with_route(&stream, &this, suc_code);
            });
            if result.is_err() {
                error!("servre handle err={:?}", result.unwrap_err());
            }
            LoopStatus::Continue
        });
    }

    pub fn stop(&mut self) {
        self._stop = true;
    }
}

impl Server for HttpServer {
    fn start(self) {
        self.start_base(StatusCode::Ok)
    }
}

impl Default for HttpServer {
    fn default() -> Self {
        Self {
            before: _none_before,
            after: _none_after,
            header: HeaderType::default(),
            _stop: false,
            tcp_svr: TcpServer::default(),
        }
    }
}

impl Debug for HttpServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpServer")
            .field("_tcp_svr", &self.tcp_svr)
            .field("header", &self.header)
            .field("before", &"[before_func]")
            .field("after", &"[after_func]")
            .field("_stop", &self._stop)
            .finish()
    }
}

fn handler_with_route(stream: &TcpStream, server: &HttpServer, suc_code: StatusCode) {
    let mut br = BufReader::new(stream);
    let buf = header::read_header(&mut br);
    // trace!("\nreq={:?}", buf);
    //读取header
    let req = HttpRequest::new(&buf);
    if req.is_err() {
        back(stream, StatusCode::InternalServerError, req.unwrap_err());
        return;
    }
    let mut req = req.unwrap();
    let req_header = req.get_header().clone();
    let method = req.get_method();
    //读取body
    let code = req.body_mut().obtain_body(&mut br, &req_header, method);
    if code.is_err() {
        back(stream, StatusCode::BadRequest, code.unwrap_err());
        return;
    }

    let mut rsp = HttpResponse::default();
    //方法前执行
    let check = (server.before)(&req, &mut rsp);
    if !check {
        (server.after)(&req, &mut rsp);
        back(stream, StatusCode::Unauthorized, rsp.get_body().to_string());
        return;
    }

    // 根据路由执行
    // 预备执行
    let url = req.get_url().clone();

    let func = route::fun(method, &url);
    //没注册路由
    if func.is_none() {
        back(stream, StatusCode::NotFound, "Not Found");
        return;
    }
    //执行方法
    func.unwrap()(&req, &mut rsp);

    //方法后执行
    (server.after)(&req, &mut rsp);

    rsp.headers_mut().append(&mut server.header.clone());
    //回包
    back_with_header(
        stream,
        suc_code,
        rsp.get_body().to_string(),
        rsp.headers_mut(),
    );
}

pub(crate) fn back_with_header<T: std::fmt::Debug>(
    stream: &TcpStream,
    status_code: StatusCode,
    text: T,
    header: &mut HeaderType,
) {
    let rp = http_handler::response(status_code, text, header);
    // trace!("\nrsp={}", rp);
    write_msg(stream, rp.as_bytes());
}

pub(crate) fn write_msg(stream: &TcpStream, msg: &[u8]) {
    let mut bw = BufWriter::new(stream);
    let _wsize = match bw.write(msg) {
        Ok(len) => len,
        Err(e) => {
            error!("e={:?}", e);
            0
        }
    };
    // trace!("_wsize={}", _wsize);
    bw.flush().unwrap();
}

//处理回包
pub(crate) fn back<T: std::fmt::Debug>(stream: &TcpStream, status_code: StatusCode, text: T) {
    back_with_header(stream, status_code, text, &mut BTreeMap::default())
}

fn _none_before(_req: &HttpRequest, _rsp: &mut HttpResponse) -> bool {
    true
}
fn _none_after(_req: &HttpRequest, _rsp: &mut HttpResponse) {}

#[test]
fn test() {
    cm_log::log_init(common::LevelFilter::Trace);

    super::route::add_get_route("/", |_req, rsp| {
        rsp.set_body("Get<h1>Hello World!</h1>");
    });
    super::route::add_post_route("/tools/img-merge", |req, rsp| {
        trace!(
            "body={:?}",
            &String::from_utf8_lossy(&req.get_body().get_u8s())[..9000]
        );
        rsp.set_body("Post<h1>Hello World!</h1>");
    });
    super::route::print_routes();
    let mut svr = HttpServer::default();
    let mut tcp_svr = TcpServer::default();
    tcp_svr.addr = "127.0.0.1:7879";
    svr.tcp_svr = tcp_svr;
    svr.after = |_, rsp| {
        rsp.set_header("Access-Control-Allow-Origin", "*");
        rsp.set_header("Access-Control-Allow-Headers", "*");
    };
    svr.start();
}
