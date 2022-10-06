use super::{Body, Header};
use common::errs::SResult;

use std::collections::BTreeMap;
use std::sync::atomic::AtomicUsize;

use bytes::BytesMut;

//接收到的请求的请求id
static ACCEPT_REQ_ID: AtomicUsize = AtomicUsize::new(1);

//////////////////////////////////////////////////////
/// 请求方法 空格 URI 空格 协议版本 回车 换行
/// 请求头名 : 请求头值 回车 换行
/// ...
/// 请求头名 : 请求头值 回车 换行
/// 回车 换行
/// 正文
/////////////////////////////////////////////////////
#[derive(Debug, Default, PartialEq)]
pub struct HttpRequest {
    _req_id: usize,
    //请求方法
    _method: HttpMethod,
    //请求url
    _url: String,
    //query
    _query: String,
    //协议版本
    _version: String,
    //请求头
    _header: Header,
    //请求数据
    _body: Body,
}

impl HttpRequest {
    //②
    // head: http请求头
    pub(crate) fn new(head: &str) -> SResult<Self> {
        let mut req = HttpRequest::default();
        //每次加一，此处原子操作返回的是加之前的值
        req._req_id = ACCEPT_REQ_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let first = req._header.parse_head(head);
        if first.is_err() {
            return Err(first.unwrap_err());
        }
        let first_line = first.unwrap();
        //请求方法
        req._method = HttpMethod::from(first_line[0]);
        let two = first_line[1];
        let index = two.find("?");

        //请求url和query
        if index.is_none() {
            req._url = two.to_string();
        } else {
            let index = index.unwrap();
            let tmp = String::from(&two[..index]);
            req._url = tmp;
            req._query = two[index + 1..].to_string();
        }

        //协议版本
        req._version = first_line[2].to_string();
        Ok(req)
    }

    pub fn get_req_id(&self) -> usize {
        self._req_id
    }
    pub fn get_url(&self) -> &String {
        &self._url
    }
    pub fn get_query(&self) -> &String {
        &self._query
    }
    pub fn get_method(&self) -> HttpMethod {
        self._method
    }
    pub(crate) fn body_mut(&mut self) -> &mut Body {
        &mut self._body
    }
    pub(crate) fn get_body(&self) -> &Body {
        &self._body
    }
    pub fn get_body_bytes(&self) -> &BytesMut {
        self._body.get_body()
    }
    pub fn get_body_u8s(&self) -> Vec<u8> {
        self._body.get_u8s()
    }
    pub fn get_header(&self) -> &Header {
        &self._header
    }
    pub fn analyze_form(&self) -> BTreeMap<String, String> {
        self.get_body().analyze_form(&self._header)
    }
    pub fn analyze_param(&self) -> BTreeMap<String, String> {
        self.get_body().analyze_param()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    HEAD,
    CONNECT,
    TRACE,
}

impl From<&str> for HttpMethod {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "OPTIONS" => HttpMethod::OPTIONS,
            "HEAD" => HttpMethod::HEAD,
            "CONNECT" => HttpMethod::CONNECT,
            "TRACE" => HttpMethod::TRACE,
            _ => HttpMethod::GET,
        }
    }
}

impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod::GET
    }
}

#[allow(unused)]
#[derive(Debug, Default, PartialEq, Eq)]
struct ReqHeader {
    //(req)可接受的响应内容类型，如 Accept: text/plain
    accept: String,
    //(req)可接受的字符集，如Accept-Charset: utf-8
    accept_charset: String,
    //(req)可接受的响应内容的编码方式。如Accept-Encoding: gzip, deflate
    accept_encoding: String,
    //(req)接受语言，如Accept-Language: en-US
    accept_language: String,
    //	 用于表示HTTP协议中需要认证资源的认证信息，如Authorization: Basic OSdjJGRpbjpvcGVuIANlc2SdDE==
    authorization: String,
    //(req/rsp)用来指定当前的请求/回复中的，是否使用缓存机制。Cache-Control: no-cache
    cache_control: String,
    // 客户端（浏览器）想要优先使用的连接类型 Connection: keep-alive、Connection: Upgrade
    connection: String,
    //由之前服务器通过Set-Cookie设置的一个HTTP协议Cookie，Cookie: $Version=1; Skin=new;
    cookie: String,
    //用来指明发送给接收方的消息主体的大小，用十进制数字表示的八位元组的数目，Content-Length: 348
    content_length: usize,
    //请求体的MIME类型 （用于POST和PUT请求中），Content-Type: application/x-www-form-urlencoded
    content_type: String,
    //表示客户端要求服务器做出特定的行为，Expect: 100-continue
    except: String,
    //发起此请求的用户的邮件地址，From: user@itbilu.com
    from: String,
    //表示服务器的域名以及服务器所监听的端口号。如果所请求的端口是对应的服务的标准端口（80），则端口号可以省略。Host: www.itbilu.com:80
    host: String,
    //限制该消息可被代理及网关转发的次数。Max-Forwards: 10
    max_forwards: usize,
    //发起一个针对跨域资源共享的请求（该请求要求服务器在响应中加入一个Access-Control-Allow-Origin的消息头，表示访问控制所允许的来源）。
    //Origin: http://www.itbilu.com
    origin: String,
    //表示浏览器所访问的前一个页面，可以认为是之前访问页面的链接将浏览器带到了当前页面。
    //Referer其实是Referrer这个单词，但RFC制作标准时给拼错了，后来也就将错就错使用Referer了。
    //Referer: http://itbilu.com/nodejs
    referer: String,
    //浏览器的身份标识字符串，User-Agent: Mozilla/……
    user_agent: String,
    //告诉服务器，这个请求是由哪些代理发出的。Via: 1.0 fred, 1.1 itbilu.com.com (Apache/1.1)
    via: String,
    //一个一般性的警告，表示在实体内容体中可能存在错误。Warning: 199 Miscellaneous warning
    warning: String,
}
