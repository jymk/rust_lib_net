use std::{
    collections::BTreeMap,
    io::{BufReader, BufWriter, Write},
    net::TcpStream,
};

use crate::{http::http_handler, HttpRequest, HttpResponse, StatusCode};
use common::status::LoopStatus;

use super::{
    header::{self, HeaderType},
    route,
};

pub type BeforeType = fn(&HttpRequest, &mut HttpResponse) -> bool;
pub type AfterType = fn(&HttpRequest, &mut HttpResponse);

#[derive(Clone)]
pub struct Server<'a> {
    // 绑定地址(包括端口)
    _addr: &'a str,
    _header: HeaderType,
    // 方法前执行
    _before: BeforeType,
    // 方法后执行
    _after: AfterType,
    _stop: bool,
}

impl<'a> Server<'a> {
    pub fn with_addr(&mut self, addr: &'a str) -> &mut Self {
        self._addr = addr;
        self
    }

    pub fn with_before(&mut self, before: BeforeType) -> &mut Self {
        self._before = before;
        self
    }

    pub fn with_after(&mut self, after: AfterType) -> &mut Self {
        self._after = after;
        self
    }

    pub fn with_header(&mut self, header: HeaderType) -> &mut Self {
        self._header = header;
        self
    }

    /// 普通web服务启动
    pub fn start(&self) {
        self.start_base(StatusCode::Ok)
    }

    /// 服务启动
    pub fn start_base(&self, suc_code: StatusCode) {
        crate::tcp::server::TcpServer::default()
            .with_addr(self._addr)
            .start(|stream| {
                if self._stop {
                    return LoopStatus::Break;
                }
                let result = std::panic::catch_unwind(|| {
                    handler_with_route(&stream, self, suc_code);
                });
                if result.is_err() {
                    eprintln!("servre handle err={:?}", result.unwrap_err());
                }
                LoopStatus::Continue
            });
    }

    pub fn stop(&mut self) {
        self._stop = true;
    }
}

impl<'a> Default for Server<'a> {
    fn default() -> Self {
        Self {
            _addr: "127.0.0.1:7878",
            _before: _none_before,
            _after: _none_after,
            _header: HeaderType::default(),
            _stop: false,
        }
    }
}

fn handler_with_route(stream: &TcpStream, server: &Server, suc_code: StatusCode) {
    let mut br = BufReader::new(stream);
    let buf = header::read_head(&mut br);
    // println!("\nreq={:?}", buf);
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
    let check = (server._before)(&req, &mut rsp);
    if !check {
        (server._after)(&req, &mut rsp);
        back(stream, StatusCode::Unauthorized, rsp.get_body().to_string());
        return;
    }

    // 根据路由执行
    // 预备执行
    let url = req.get_url().clone();

    //没注册路由
    if route::no_has_fun(method, &url) {
        back(stream, StatusCode::NotFound, "Not Found");
        return;
    }
    //执行方法
    route::fun(method, &url)(&req, &mut rsp);

    //方法后执行
    (server._after)(&req, &mut rsp);

    rsp.headers_mut().append(&mut server._header.clone());
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
    // println!("\nrsp={}", rp);
    write_msg(stream, rp.as_bytes());
}

pub(crate) fn write_msg(stream: &TcpStream, msg: &[u8]) {
    let mut bw = BufWriter::new(stream);
    let _wsize = match bw.write(msg) {
        Ok(len) => len,
        Err(e) => {
            eprintln!("e={:?}", e);
            0
        }
    };
    // println!("_wsize={}", _wsize);
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
    super::route::add_get_route("/", |req, rsp| {
        rsp.set_body("<h1>Hello World!</h1>");
    });
    super::route::add_post_route("/", |req, rsp| {
        println!("body={:?}", req.get_body());
        rsp.set_body("<h1>Hello World!</h1>");
    });
    super::route::print_routes();
    Server::default().start();
}
