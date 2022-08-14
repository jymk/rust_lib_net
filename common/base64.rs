use std::{
    fs::File,
    io::{Read, Result},
};

/// 文件转u8 vec
pub fn file_to_u8s(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buf = Vec::default();
    file.read_to_end(&mut buf).unwrap();
    Ok(buf)
}

//编码表
const ENCODE_TABLE: &[u8] =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
const MASK1: usize = 0x3F;
/// 编码，不带data:image/png;base64,头
pub fn base64_encode(data: &[u8]) -> String {
    let mut res = String::default();

    let mut line_len = 0;
    let mut tmp = [0u8; 4];
    let mut i = 0;
    while i < data.len() / 3 {
        tmp[1] = data[i * 3];
        tmp[2] = data[i * 3 + 1];
        tmp[3] = data[i * 3 + 2];
        res.push(ENCODE_TABLE[tmp[1] as usize >> 2] as char);
        res.push(
            ENCODE_TABLE[(((tmp[1] as usize) << 4) | ((tmp[2] as usize) >> 4)) & MASK1] as char,
        );
        res.push(
            ENCODE_TABLE[(((tmp[2] as usize) << 2) | ((tmp[3] as usize) >> 6)) & MASK1] as char,
        );
        res.push(ENCODE_TABLE[tmp[3] as usize & MASK1] as char);
        line_len += 4;
        if line_len == 76 {
            res.push_str("\r\n");
            line_len = 0;
        }
        i += 1;
    }
    //对剩余数据进行编码
    let mold: usize = data.len() % 3;
    match mold {
        1 => {
            tmp[1] = data[i * 3];
            res.push(ENCODE_TABLE[((tmp[1] & 0xFC) as usize) >> 2] as char);
            res.push(ENCODE_TABLE[((tmp[1] & 0x03) as usize) << 4] as char);
            res.push_str("==");
        }
        2 => {
            tmp[1] = data[i * 3];
            tmp[2] = data[i * 3 + 1];
            res.push(ENCODE_TABLE[((tmp[1] & 0xFC) as usize) >> 2] as char);
            res.push(
                ENCODE_TABLE[(((tmp[1] & 0x03) as usize) << 4) | (((tmp[2] & 0xF0) as usize) >> 4)]
                    as char,
            );
            res.push(ENCODE_TABLE[((tmp[2] & 0x0F) as usize) << 2] as char);
            res.push('=');
        }
        _ => {}
    }
    res
}

/// 文件转base64
pub fn file_to_base64(path: &str) -> Result<String> {
    // 读取文件内容为Vec<u8>
    let u8_vec = file_to_u8s(path)?;
    // 将[u8]转为base64
    Ok(base64_encode(&u8_vec))
}
