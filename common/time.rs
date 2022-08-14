use std::time::{Duration, SystemTime};

pub fn now() -> SystemTime {
    SystemTime::now()
}

pub fn now_drt() -> Duration {
    now().duration_since(std::time::UNIX_EPOCH).unwrap()
}

pub fn now_milli_str() -> String {
    now_drt().as_millis().to_string()
}

/// t与现在相距时间
pub fn since(t: SystemTime) -> Duration {
    now().duration_since(t).unwrap()
}

#[test]
fn test() {
    println!("now={:?}", now_drt());
}
