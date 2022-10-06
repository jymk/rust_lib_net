use common::errs::{sresult_from_err, SResult};

/// 请求格式错误
pub(crate) const ERR_FORMAT: &'static str = "格式错误";
/// 服务器错误
pub(crate) const ERR_SERVER_INTERNAL: &'static str = "服务器内部错误";

/// 格式错误
pub(crate) fn err_format<T>() -> SResult<T> {
    sresult_from_err(ERR_FORMAT)
}

/// 服务器内部错误
pub(crate) fn err_server_internal<T>() -> SResult<T> {
    sresult_from_err(ERR_SERVER_INTERNAL)
}
