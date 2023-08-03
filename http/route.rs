use std::{
    collections::BTreeMap,
    fmt::Debug,
    sync::{Arc, RwLock},
};

#[allow(unused_imports)]
use common::{cm_log, trace};

use crate::route::Route;

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
/// 路由缓存
type RouteMapType = BTreeMap<HttpRoute, ValType>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct HttpRoute(Route, HttpMethod);

impl HttpRoute {
    pub(crate) fn new(method: HttpMethod, url: &str) -> Self {
        Self(Route::new(url), method)
    }
}

impl Debug for HttpRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HttpRoute")
            .field(&self.0.get_url())
            .field(&self.1)
            .finish()
    }
}

pub(crate) fn fun(method: HttpMethod, url: &str) -> Option<ValType> {
    let instance = _get_route_instance();
    let r = instance.read().unwrap();
    let func = r.get(&HttpRoute::new(method, url));
    if func.is_none() {
        return None;
    }
    Some(*func.unwrap())
}

/// 下面两个函数限制HttpMethod
pub fn add_get_route(url: &'static str, func: ValType) {
    _get_route_instance()
        .write()
        .unwrap()
        .insert(HttpRoute::new(HttpMethod::GET, url), func);
}

pub fn add_post_route(url: &'static str, func: ValType) {
    _get_route_instance()
        .write()
        .unwrap()
        .insert(HttpRoute::new(HttpMethod::POST, url), func);
    _get_route_instance()
        .write()
        .unwrap()
        .insert(HttpRoute::new(HttpMethod::OPTIONS, url), |_, _| {});
}

#[allow(unused)]
pub fn print_routes() {
    let instance = _get_route_instance();
    let routes = instance.read().unwrap();
    for route in &*routes {
        trace!("{:?}", route.0);
    }
}

#[test]
fn test() {
    cm_log::log_init(common::LevelFilter::Debug);

    add_get_route("/", |_req, _rsp| {});
    add_post_route("/", |_req, _rsp| {});
    print_routes();
}
