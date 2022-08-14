use std::{
    collections::BTreeMap,
    fmt::Debug,
    sync::{Arc, RwLock},
};

use crate::common::strings::str_to_static;

use super::{
    req::{HttpMethod, HttpRequest},
    rsp::HttpResponse,
};

fn _get_route_instance() -> Arc<RwLock<RouteMapType>> {
    static mut ROUTES: Option<Arc<RwLock<RouteMapType>>> = None;
    unsafe {
        ROUTES
            .get_or_insert_with(|| Arc::new(RwLock::new(RouteMapType::default())))
            .clone()
    }
}

/// 执行函数
type ValType = fn(&HttpRequest, &mut HttpResponse);

type RouteMapType = BTreeMap<Route, ValType>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Route {
    _method: HttpMethod,
    _url: &'static str,
}

impl Route {
    pub(crate) fn new(method: HttpMethod, url: &str) -> Self {
        Self {
            _method: method,
            _url: str_to_static(url.to_string()),
        }
    }

    pub(crate) fn new2(method: HttpMethod, url: &'static str) -> Self {
        Self {
            _method: method,
            _url: url,
        }
    }
}

pub(crate) fn fun(method: HttpMethod, url: &str) -> ValType {
    let instance = _get_route_instance();
    let r = instance.read().unwrap();
    let fun = r.get(&Route::new(method, url));
    if fun.is_none() {
        return _none_fun;
    }
    *fun.unwrap()
}

pub(crate) fn no_has_fun(method: HttpMethod, url: &str) -> bool {
    let instance = _get_route_instance();
    let r = instance.read().unwrap();
    let fun = r.get(&Route::new(method, url));
    // print_routes();
    // println!("cur_method={:?}, cur_url={}", method, url);
    fun.is_none()
}

fn _none_fun(_: &HttpRequest, _: &mut HttpResponse) {}

impl Default for Route {
    fn default() -> Self {
        Self {
            _method: HttpMethod::default(),
            _url: "",
        }
    }
}

pub fn add_get_route(url: &'static str, fun: ValType) {
    _get_route_instance()
        .write()
        .unwrap()
        .insert(Route::new2(HttpMethod::GET, url), fun);
}

pub fn add_post_route(url: &'static str, fun: ValType) {
    _get_route_instance()
        .write()
        .unwrap()
        .insert(Route::new2(HttpMethod::POST, url), fun);
}

#[allow(unused)]
pub fn print_routes() {
    let instance = _get_route_instance();
    let routes = instance.read().unwrap();
    for route in &*routes {
        println!("{:?}", route.0);
    }
}

#[test]
fn test() {
    // for r in routes() {
    //     println!("route={:?}", r.0);
    // }
}
