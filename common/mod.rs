#![allow(unused)]
pub mod base64;
pub mod errs;
mod status;
pub mod strings;
pub mod time;

pub use status::LoopStatus;

/// BTreeMap
/// let mut m = btreemap(1 => 2, 3 => 4);
#[macro_export]
macro_rules! btreemap {
    ($($($x: expr => $y: expr) + $(,)?)*) => {
        {
            let mut m = ::std::collections::BTreeMap::new();
            $($(
                m.insert($x, $y);
            )*)*
            m
        }
    };
}
