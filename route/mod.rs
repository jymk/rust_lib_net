//! 这里主要为了实现route的比较

use std::fmt::{Debug, Display};

#[allow(unused_imports)]
use common::debug;
#[allow(unused_imports)]
use common::{cm_log, strings::str_to_static};

/// 此处route以ip:port之后的第一个字符开头(是否含有/都行)
/// 不支持..寻上一级
#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub struct Route {
    _url: &'static str,
}

impl Route {
    pub(crate) fn new(url: &str) -> Self {
        Self::new_static(str_to_static(url.to_string()))
    }

    pub(crate) fn new_static(url: &'static str) -> Self {
        Self { _url: url }
    }

    pub(crate) fn get_url(&self) -> &'static str {
        self._url
    }
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        let mut urls = self._url.split("/").collect::<Vec<_>>();
        let mut other_urls = other._url.split("/").collect::<Vec<_>>();
        // trace!("before remove: urls={:?}, other={:?}", urls, other_urls);
        // ./xxx/xxx/{xx}/xxx
        _remove_useless(&mut urls);
        _remove_useless(&mut other_urls);
        // trace!("after remove: urls={:?}, other={:?}", urls, other_urls);

        urls == other_urls
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// 去除route中无用字符
fn _remove_useless(urls: &mut Vec<&str>) {
    let mut i = 0;
    while i < urls.len() {
        let item = urls[i];
        match item.as_bytes() {
            [b'.', b'.', ..] => {
                urls.remove(i);
                continue;
            }
            [b'.', ..] => {
                urls.remove(i);
                continue;
            }
            [b'{', .., b'}'] => {
                urls.remove(i);
                continue;
            }
            [] => {
                urls.remove(i);
                continue;
            }
            _ => {}
        }
        i += 1;
    }
}

#[test]
fn test_route_eq() {
    cm_log::log_init(common::LevelFilter::Debug);

    fn tnew(s: &str) -> Route {
        Route::new(s)
    }

    assert_eq!(tnew("../abc"), tnew("/abc/.."));
    assert_eq!(tnew("./abc"), tnew("/abc"));
    assert_eq!(tnew("abc/{id}/def"), tnew("/abc/{ip}/def"));
    assert_ne!(tnew("abc/{id}/def"), tnew("/abc/{ip/def}"));
}
