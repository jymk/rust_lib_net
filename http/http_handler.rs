use std::{
    collections::BTreeMap,
    io::{BufReader, BufWriter, Write},
    net::TcpStream,
    time::Duration,
};

use crate::{
    http::{header, Header},
    HeaderType,
};
use common::{error, strings, trace};

use super::rsp::StatusCode;

#[allow(unused)]
pub(crate) fn send(addr: &str, http_txt: &[u8]) {
    let mut stream = TcpStream::connect(addr).expect("connect failed");
    stream.set_read_timeout(Some(Duration::from_secs(2)));
    stream.set_write_timeout(Some(Duration::from_secs(2)));
    let mut bw = BufWriter::new(&stream);
    let wsize = match bw.write(http_txt) {
        Ok(x) => x,
        Err(e) => {
            error!("e={:?}", e);
            0
        }
    };
    //特别重要，flush后才能读
    bw.flush().unwrap();
    trace!("wsize={}", wsize);
    let mut br = BufReader::new(&stream);
    let head = header::read_header(&mut br);
    let mut rsp = super::rsp::HttpResponse::new(&head);
    match rsp.as_mut() {
        Ok(x) => trace!(
            "header={:?}, body={:?}",
            x.headers(),
            String::from_utf8(x.get_body().get_u8s()),
        ),
        Err(e) => error!("e={:?}", e),
    }
}

#[allow(unused)]
pub(crate) fn get_with_header(
    host: &str,
    url: &str,
    body: &str,
    header: Option<BTreeMap<String, String>>,
) -> String {
    let mut req = String::default();
    req.push_str("GET ");
    req.push_str(url);
    req.push(' ');
    req.push_str("HTTP/2");
    req.push_str("\r\n");
    let tmp = Header::new(&mut BTreeMap::default());
    for (k, v) in tmp.headers() {
        req.push_str(&k);
        req.push_str(":");
        req.push_str(&v);
        req.push_str("\r\n");
    }
    if let Some(h) = header {
        for (k, v) in h {
            req.push_str(&k);
            req.push_str(":");
            req.push_str(&v);
            req.push_str("\r\n");
        }
    }
    req.push_str("Host");
    req.push_str(":");
    req.push_str(host);
    req.push_str("\r\n");
    req.push_str("Content-Length");
    req.push_str(":");
    req.push_str(&body.len().to_string());
    req.push_str("\r\n");

    req.push_str("\r\n");
    req.push_str(body);
    req
}

//get请求
#[allow(unused)]
pub(crate) fn get(host: &str, url: &str, body: &str) -> String {
    get_with_header(host, url, body, None)
}

//post请求
#[allow(unused)]
pub(crate) fn post(host: &str, url: &str, body: &str) -> String {
    let mut req = String::default();
    req.push_str("POST ");
    req.push_str(url);
    req.push(' ');
    req.push_str("HTTP/1.1");
    req.push_str("\r\n");
    let header = Header::new(&mut BTreeMap::default());
    for (k, v) in header.headers() {
        req.push_str(&k);
        req.push_str(":");
        req.push_str(&v);
        req.push_str("\r\n");
    }
    req.push_str("Host");
    req.push_str(":");
    req.push_str(host);
    req.push_str("\r\n");

    req.push_str("Content-Length");
    req.push_str(":");
    req.push_str(&body.len().to_string());
    req.push_str("\r\n");

    req.push_str("\r\n");
    req.push_str(body);
    trace!("req={}", req);
    req
}

pub(crate) fn response<T: std::fmt::Debug>(
    status_code: StatusCode,
    body: T,
    header: &mut HeaderType,
) -> String {
    let mut rsp = String::default();
    rsp.push_str("HTTP/1.1 ");
    rsp.push_str(&status_code.to_code_str());
    rsp.push(' ');
    rsp.push_str(&status_code.to_string());
    rsp.push_str("\r\n");
    let header = Header::new(header);
    for (k, v) in header.headers() {
        rsp.push_str(&k);
        rsp.push_str(": ");
        rsp.push_str(&v);
        rsp.push_str("\r\n");
    }

    let body_str = strings::unescape(&format!("{:?}", body));
    rsp.push_str("content-length");
    rsp.push_str(": ");
    rsp.push_str(body_str.len().to_string().as_str());
    rsp.push_str("\r\n");

    rsp.push_str("\r\n");
    rsp.push_str(&body_str);
    rsp
}

#[test]
fn test() {
    let a = String::from("\'");
    trace!("a={}", a);
    let fa = format!("{:?}", a);
    trace!("a={}", fa);
    // let ffa = format!("{:?}", fa);
    // trace!("a={}", ffa);
    // let req = get("www.baidu.com",
    // "/sugrec?prod=pc_his&from=pc_web&json=1&sid=36309_31660_36005_35910_36165_34584_35978_36345_26350_36349_36311_36061&hisdata=&_t=1651989831478&req=2&csor=0", "");
    // send("www.baidu.com:80", req.as_bytes());

    // let header = btreemap!(
    //     "X-Requested-With".to_string() => "XMLHttpRequest".to_string(),
    //     "Referer".to_string() => "https://cn.pornhub.com/".to_string(),
    //     "Cookie".to_string() => "bs=5tn137c9gqphkmavp4fm5lfv0y5hdc7l; ss=573227799567438799; platform=pc; fg_fcf2e67d6468e8e1072596aead761f2b=7943.100000; fg_ee26b76392ae0c54fbcf7c635e3da0fa=91225.100000; fg_1ad264a225f47ac0e57be2740d6aa857=10575.100000; il=v1nb6Go30V25r28BQQsYtYDOY46017N1dc18Hwy10m2mkxNjU5Mjc1MDI3LWl1dkRwOFJMeFZWQ2N2VFR1d3dnOHBrU3czWDU4SzZCdmxjV0w1Vw..; expiredEnterModalShown=1; fg_ec733c207a91321a9ed22b01850e820e=2705.100000; fg_1f595a21748e9a93d04690b86a079a2a=34869.100000; fg_7eb5e6299ee952620df104ed2e7aac86=24270.100000; atatusScript=hide; ua=f0f466a59a9efd2032e38b07d3447d1c".to_string(),
    //     "Sec-Fetch-Dest".to_string() => "empty".to_string(),
    //     "Sec-Fetch-Mode".to_string() => "cors".to_string(),
    //     "Sec-Fetch-Site".to_string() => "https://cn.pornhub.com/".to_string(),
    //     "Referer".to_string() => "same-origin".to_string(),
    //     "TE".to_string() => "trailers".to_string(),
    // );
    // let req = get_with_header("cn.pornhub.com",
    // "/front/menu_livesex?segment=straight&token=MTY1MTk5MjU1MZIU4aJmZ_Lnw46E1gpBWpjhL8-Tf9L41Oo21t_rCrISoqW8x8ocwxXtTMl32VMM6pBrnMgaZJmenbdRDlU9p64.?",
    // "",
    // Some(header));
    // trace!("req={}", req);
    // send("https://cn.pornhub.com:80", req.as_bytes());

    // let req = post(
    //     "jymkyu.top:443",
    //     "/api/file/load-root",
    //     "{\"pwd\": \"croissant\"}",
    // );
    // send("jymkyu.top:443", req.as_bytes());
    // let req = post(
    //     "127.0.0.1:7879",
    //     "/common/disease/queryDiseaseListByKeyword",
    //     "{\"pwd\":\"croissant\"}",
    // );
    // send("127.0.0.1:7879", req.as_bytes());
    // let req = post("baidu.com", "", "");
    // send("www.baidu.com:443", req.as_bytes());
}
