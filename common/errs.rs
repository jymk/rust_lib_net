use std::fmt::{Debug, Display};

/// 请求格式错误
pub const ERR_FORMAT: &'static str = "格式错误";
/// 服务器错误
pub const ERR_SERVER_INTERNAL: &'static str = "服务器内部错误";

/// 格式错误
pub fn err_format<T>() -> SResult<T> {
    sresult_from_err(ERR_FORMAT)
}

/// 服务器内部错误
pub fn err_server_internal<T>() -> SResult<T> {
    sresult_from_err(ERR_SERVER_INTERNAL)
}

/// 字符串异常
#[derive(Debug, Clone)]
pub struct SError(String);

impl From<std::io::Error> for SError {
    fn from(e: std::io::Error) -> Self {
        Self(e.to_string())
    }
}

impl From<&str> for SError {
    fn from(e: &str) -> Self {
        Self(e.to_string())
    }
}

impl From<String> for SError {
    fn from(e: String) -> Self {
        Self(e)
    }
}

impl ToString for SError {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub type SResult<T> = std::result::Result<T, SErrs>;

/// 从from SErrs生成SResult
pub fn sresult_from_err<T: Into<SErrs>, U>(t: T) -> SResult<U> {
    SErrs::from(t.into()).to_sresult()
}

pub fn sresult_to_string<T: std::fmt::Debug>(res: SResult<T>) -> String {
    if res.is_err() {
        res.unwrap_err().to_string()
    } else {
        String::default()
    }
}

#[derive(Debug)]
pub enum SErrs {
    IoError(std::io::Error),
    FromU8Error(std::string::FromUtf8Error),
    SError(SError),
    NullError,
}

impl SErrs {
    pub fn to_sresult<T>(self) -> SResult<T> {
        Err(self)
    }
}

impl From<std::io::Error> for SErrs {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<SError> for SErrs {
    fn from(e: SError) -> Self {
        Self::SError(e)
    }
}

impl<T: Debug> From<std::io::Result<T>> for SErrs {
    fn from(r: std::io::Result<T>) -> Self {
        Self::from(r.unwrap_err())
    }
}

impl From<&str> for SErrs {
    fn from(s: &str) -> Self {
        Self::SError(SError::from(s))
    }
}

impl From<String> for SErrs {
    fn from(s: String) -> Self {
        Self::SError(SError::from(s))
    }
}

impl From<std::string::FromUtf8Error> for SErrs {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::FromU8Error(e)
    }
}

impl Display for SErrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl Default for SErrs {
    fn default() -> Self {
        Self::NullError
    }
}
