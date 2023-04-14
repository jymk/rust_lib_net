use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use common::trace;

use crate::route::Route;

use super::{req::PbRequest, rsp::PbResponse};

fn _get_route_instance() -> Arc<RwLock<RouteMapType>> {
    static mut ROUTES: Option<Arc<RwLock<RouteMapType>>> = None;
    unsafe {
        ROUTES
            .get_or_insert_with(|| Arc::new(RwLock::new(RouteMapType::default())))
            .clone()
    }
}

/// 执行函数
type ValType = fn(&PbRequest, &mut PbResponse);
/// 路由缓存，server_name => {function => 执行函数}
type RouteMapType = BTreeMap<String, BTreeMap<Route, ValType>>;

#[allow(unused)]
pub fn add_route(server_name: &str, function: &str, func: ValType) {
    let instance = _get_route_instance();
    let mut w = instance.write().unwrap();
    let route_map = w.get_mut(server_name);
    if let Some(x) = route_map {
        x.insert(Route::new(function), func);
    } else {
        w.insert(
            server_name.to_string(),
            BTreeMap::from([(Route::new(function), func)]),
        );
    }
}

#[allow(unused)]
pub(crate) fn fun(server_name: &str, function: &str) -> Option<ValType> {
    let instance = _get_route_instance();
    let r = instance.read().unwrap();
    let func = r.get(server_name);
    if func.is_none() {
        return None;
    }
    let func = func.unwrap();
    let func = func.get(&Route::new(function));
    if func.is_none() {
        return None;
    }
    Some(*func.unwrap())
}

#[allow(unused)]
pub fn print_routes() {
    let instance = _get_route_instance();
    let servers = instance.read().unwrap();
    for server in &*servers {
        for route in server.1 {
            trace!("{}/{}", server.0, route.0.get_url());
        }
    }
}

#[test]
fn test_print_routes() {
    common::cm_log::log_init(common::LevelFilter::Debug);

    add_route("a.b", "function", |_req, _rsp| {});
    add_route("a.b", "function2", |_req, _rsp| {});
    add_route("a.c", "function2", |_req, _rsp| {});
    print_routes();
}
