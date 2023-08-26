use super::{header::*, Body};
#[allow(unused_imports)]
use common::{errs::SResult, trace};

//////////////////////////////////////////////////////
/// 版本 空格 状态码 空格 状态说明 回车 换行
/// 响应头名 : 响应头值 回车 换行
/// ...
/// 响应头名 : 响应头值 回车 换行
/// 回车 换行
/// 正文
/////////////////////////////////////////////////////
#[derive(Debug, Default)]
pub struct HttpResponse {
    //协议版本
    _version: String,
    //状态码
    _status_code: StatusCode,
    //状态文本
    _status_text: String,
    //响应头
    _header: Header,
    //响应数据
    _body: Body,
}

impl HttpResponse {
    pub(crate) fn new(header: &str) -> SResult<HttpResponse> {
        let mut rsp = HttpResponse::default();
        let first = rsp._header.parse_header(header);
        if first.is_err() {
            return Err(first.unwrap_err());
        }
        let first_line = first.unwrap();
        rsp._version = first_line[0].to_string();
        rsp._status_code = StatusCode::from(&first_line[1]);
        rsp._status_text = first_line[2].to_string();
        Ok(rsp)
    }

    /// 设置body
    pub fn set_body(&mut self, body: &str) {
        self._body.set_body(body);
    }
    pub fn set_body_slice(&mut self, body: &[u8]) {
        self._body.set_body(body);
    }

    pub fn set_header(&mut self, key: &str, val: &str) {
        self._header.set_directly(key, val)
    }

    pub fn headers(&self) -> &HeaderType {
        self._header.headers()
    }

    pub fn headers_mut(&mut self) -> &mut HeaderType {
        self._header.headers_mut()
    }

    pub fn get_body(&self) -> &Body {
        &self._body
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    /// 继续
    /// 如果服务器收到头信息中带有100-continue的请求，
    /// 这是指客户端询问是否可以在后续的请求中发送附件。
    /// 在这种情况下，服务器用100(SC_CONTINUE)允许客户端继续或用417 (Expectation Failed)告诉客户端不同意接受附件。
    /// 这个状态码是 HTTP 1.1中新加入的。
    Continue = 100,
    /// 转换协议
    /// 指服务器将按照其上的头信息变为一个不同的协议。这是 HTTP 1.1中新加入的。
    SwitchingProtocols = 101,
    /// 正常
    /// 一般用于相应GET和POST请求。这个状态码对servlet是缺省的;如果没有调用setStatus方法的话，就会得到200。
    Ok = 200,
    /// 已创建
    /// 表示服务器在请求的响应中建立了新文档;应在定位头信息中给出它的URL。
    Created = 201,
    /// 接受
    /// 告诉客户端请求正在被执行，但还没有处理完。
    Accepted = 202,
    /// 非官方信息
    /// 表示文档被正常的返回，但是由于正在使用的是文档副本所以某些响应头信息可能不正确。这是 HTTP 1.1中新加入的。
    NonAuthoritativeInformation = 203,
    /// 无内容
    /// 在并没有新文档的情况下，204 (SC_NO_CONTENT)确保浏览器继续显示先前的文档。
    /// 这各状态码对于用户周期性的重载某一页非常有用，并且你可以确定先前的页面是否已经更新。
    /// 但是，这种方法对通过刷新响应头信息或等价的HTML标记自动重载的页面起作用，因为它会返回一个204状态码停止以后的重载。
    /// 但基于JavaScript脚本的自动重载在这种情况下仍然需要能够起作用。
    NoContent = 204,
    /// 重置内容
    /// 虽然没有新文档但浏览器要重置文档显示。这个状态码用于强迫浏览器清除表单域。
    ResetContent = 205,
    /// 局部内容
    /// 在服务器完成了一个包含Range头信息的局部请求时被发送的。这是 HTTP 1.1中新加入的。
    PartialContent = 206,
    /// 多重选择
    /// 表示被请求的文档可以在多个地方找到，并将在返回的文档中列出来。如果服务器有首选设置，首选项将会被列于定位响应头信息中。
    MultipleChoices = 300,
    /// 所请求的文档在别的地方;文档新的URL会在定位响应头信息中给出。浏览器会自动连接到新的URL。
    MovedPermanently = 301,
    /// 找到
    /// 与301有些类似，只是定位头信息中所给的URL应被理解为临时交换地址而不是永久的。
    /// 注意：在 HTTP 1.0中，消息是临时移动(Moved Temporarily)的而不是被找到，
    /// 因此java中HttpServletResponse中的常量是SC_MOVED_TEMPORARILY不是我们以为的SC_FOUND。
    Found = 302,
    /// 参见其他信息
    /// 这个状态码和 301、302 相似，只是如果最初的请求是 POST，那么新文档(在定位头信息中给出)要用 GET 找回。
    /// 这个状态码是新加入 HTTP 1.1中的。
    SeeOther = 303,
    /// 未修正
    /// 当客户端有一个缓存的文档，通过提供一个 If-Modified-Since 头信息可指出客户端只希望文档在指定日期之后有所修改时才会重载此文档，用这种方式可以进行有条件的请求。
    /// 304 (SC_NOT_MODIFIED)是指缓冲的版本已经被更新并且客户端应刷新文档。另外，服务器将返回请求的文档及状态码 200。
    NotModified = 304,
    /// 使用代理
    /// 表示所请求的文档要通过定位头信息中的代理服务器获得。这个状态码是新加入 HTTP 1.1中的。
    UseProxy = 305,
    /// 临时重定向
    /// 浏览器处理307状态的规则与302相同。307状态被加入到 HTTP 1.1中是由于许多浏览器在收到302响应时即使是原始消息为POST的情况下仍然执行了错误的转向。
    /// 只有在收到303响应时才假定浏览器会在POST请求时重定向。
    /// 添加这个新的状态码的目的很明确：在响应为303时按照GET和POST请求转向;而在307响应时则按照GET请求转向而不是POST请求。
    TemporatyRedirect = 307,
    /// 错误请求
    /// 指出客户端请求中的语法错误。
    BadRequest = 400,
    /// 未授权
    /// 表示客户端在授权头信息中没有有效的身份信息时访问受到密码保护的页面。这个响应必须包含一个WWW-Authenticate的授权信息头。
    Unauthorized = 401,
    /// 禁止
    /// 除非拥有授权否则服务器拒绝提供所请求的资源。这个状态经常会由于服务器上的损坏文件或目录许可而引起。
    Forbidden = 403,
    /// 未找到
    /// 告诉客户端所给的地址无法找到任何资源。它是表示“没有所访问页面”的标准方式。
    /// 默认情况下，IE5忽略服务端生成的错误提示页面。
    NotFound = 404,
    /// 方法未允许
    /// 指出请求方法(GET, POST, HEAD, PUT, DELETE, 等)对某些特定的资源不允许使用。该状态码是新加入 HTTP 1.1中的。
    MethodNotAllowed = 405,
    /// 无法访问
    /// 表示请求资源的MIME类型与客户端中Accept头信息中指定的类型不一致。
    NotAcceptable = 406,
    /// 代理服务器认证要求
    /// 与401状态有些相似，只是这个状态用于代理服务器。该状态指出客户端必须通过代理服务器的认证。
    /// 代理服务器返回一个Proxy-Authenticate响应头信息给客户端，这会引起客户端使用带有Proxy-Authorization请求的头信息重新连接。
    /// 该状态码是新加入 HTTP 1.1中的。
    ProxyAuthenticationRequired = 407,
    /// 请求超时
    /// 指服务端等待客户端发送请求的时间过长。该状态码是新加入 HTTP 1.1中的。
    RequestTimeout = 408,
    /// 冲突
    /// 该状态通常与PUT请求一同使用，409 (SC_CONFLICT)状态常被用于试图上传版本不正确的文件时。该状态码是新加入 HTTP 1.1中的。
    Conflict = 409,
    /// 已经不存在
    /// 告诉客户端所请求的文档已经不存在并且没有更新的地址。
    /// 410状态不同于404，410是在知道文档已被移走的情况下使用，而404则用于未知原因的无法访问。
    /// 该状态码是新加入 HTTP 1.1中的。
    Gone = 410,
    /// 需要数据长度
    /// 表示服务器不能处理请求(假设为带有附件的POST请求)，除非客户端发送Content-Length头信息指出发送给服务器的数据的大小。
    /// 该状态是新加入 HTTP 1.1的。
    LengthRequired = 411,
    /// 先决条件错误
    /// 指出请求头信息中的某些先决条件是错误的。该状态是新加入 HTTP 1.1的。
    PreconditionFailed = 412,
    /// 请求实体过大
    /// 告诉客户端现在所请求的文档比服务器现在想要处理的要大。
    /// 如果服务器认为能够过一段时间处理，则会包含一个Retry-After的响应头信息。该状态是新加入 HTTP 1.1的。
    RequestEntityTooLarge = 413,
    /// 请求URI过长
    /// 状态用于在URI过长的情况时。这里所指的“URI”是指URL中主机、域名及端口号之后的内容。
    RequestURITooLong = 414,
    /// 不支持的媒体格式
    /// 意味着请求所带的附件的格式类型服务器不知道如何处理。该状态是新加入 HTTP 1.1的。
    UnsupportedMediaType = 415,
    /// 请求范围无法满足
    /// 表示客户端包含了一个服务器无法满足的Range头信息的请求。该状态是新加入 HTTP 1.1的。
    RequestedRangeNotSatisfiable = 416,
    /// 期望失败
    /// 如果服务器得到一个带有100-continue值的Expect请求头信息，这是指客户端正在询问是否可以在后面的请求中发送附件。
    /// 在这种情况下，服务器也会用该状态(417)告诉浏览器服务器不接收该附件或用100 (SC_CONTINUE)状态告诉客户端可以继续发送附件。
    /// 该状态是新加入 HTTP 1.1的。
    ExpectationFailed = 417,
    /// 内部服务器错误
    /// 常用的“服务器错误”状态。该状态经常由CGI程序引起也可能(但愿不会如此!)由无法正常运行的或返回头信息格式不正确的servlet引起。
    InternalServerError = 500,
    /// 未实现
    /// 告诉客户端服务器不支持请求中要求的功能。例如，客户端执行了如PUT这样的服务器并不支持的命令。
    NotImplemented = 501,
    /// 错误的网关
    /// 被用于充当代理或网关的服务器;该状态指出接收服务器接收到远端服务器的错误响应。
    BadGateway = 502,
    /// 服务无法获得
    /// 表示服务器由于在维护或已经超载而无法响应。例如，如果某些线程或数据库连接池已经没有空闲则servlet会返回这个头信息。
    /// 服务器可提供一个Retry-After头信息告诉客户端什么时候可以在试一次。
    ServiceUnavailable = 503,
    /// 网关超时
    /// 该状态也用于充当代理或网关的服务器;它指出接收服务器没有从远端服务器得到及时的响应。该状态是新加入 HTTP 1.1的。
    GatewayTimeout = 504,
    /// 不支持的 HTTP 版本
    /// 说服务器并不支持在请求中所标明 HTTP 版本。该状态是新加入 HTTP 1.1的。
    HTTPVersionNotSupported = 505,
}

impl StatusCode {
    pub fn get_code(&self) -> i32 {
        *self as i32
    }

    pub fn to_code_str(&self) -> String {
        self.get_code().to_string()
    }
}

impl From<&str> for StatusCode {
    fn from(s: &str) -> Self {
        unsafe { std::mem::transmute(s.parse::<i32>().unwrap_or(500)) }
    }
}

impl From<&String> for StatusCode {
    fn from(s: &String) -> Self {
        Self::from(s.as_str())
    }
}

impl ToString for StatusCode {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        Self::Ok
    }
}

#[allow(unused)]
#[derive(Debug, Default)]
struct RspHeader {
    //(rsp)临时，指定哪些网站可以跨域源资源共享，Access-Control-Allow-Origin: *
    access_control_allow_origin: String,
    //(rsp)响应对象在代理缓存中存在的时间，以秒为单位，Age: 12
    age: usize,
    //(rsp)对于特定资源的有效动作，Allow: GET, HEAD
    allow: String,
    //(req/rsp)用来指定当前的请求/回复中的，是否使用缓存机制。Cache-Control: no-cache
    cache_control: String,
    //(rsp)响应资源所使用的编码类型。Content-Encoding: gzip
    content_encoding: String,
    //响应内容所使用的语言，Content-Language: zh-cn
    contetn_language: String,
    //以8进制表示的请求体的长度，Content-Length: 348
    content_length: usize,
    //请求体的MIME类型 （用于POST和PUT请求中），Content-Type: application/x-www-form-urlencoded
    content_type: String,
    //指定一个日期/时间，超过该时间则认为此回应已经过期，Expires: Thu, 01 Dec 1994 16:00:00 GMT
    expires: String,
    //请求的对象的最后修改日期，Last-Modified: Dec, 26 Dec 2015 17:30:00 GMT
    last_modified: String,
    //用于重定向，或者当一个新的资源被创建时。默认会在5秒后刷新重定向。Refresh: 5; url=http://itbilu.com
    refresh: String,
    //如果某个实体临时不可用，那么此协议头用于告知客户端稍后重试。其值可以是一个特定的时间段(以秒为单位)或一个超文本传输协议日期。
    //Retry-After: 120
    //Retry-After: Dec, 26 Dec 2015 17:30:00 GMT
    retry_after: String,
    //服务器的名称，Server: nginx/1.6.3
    server: String,
    //设置HTTP cookie，Set-Cookie: UserID=itbilu; Max-Age=3600; Version=1
    set_cookie: String,
    //通用网关接口的响应头字段，用来说明当前HTTP连接的响应状态。Status: 200 OK
    status: String,
    //表示在请求获取这个实体时应当使用的认证模式。WWW-Authenticate: Basic
    www_authenticate: String,
}

#[test]
fn test() {
    trace!("{}", StatusCode::InternalServerError.to_string());
}
