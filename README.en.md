[TOC]
# lib_net

#### Description
about net


## Use  

### HttpServer  

```rust
// 创建服务
let mut svr = HttpServer::default();
// 指定启动地址
svr.addr = "url";
// 指定方法前执行（是这个方法签名，但不一定这么写）
svr.before = before(&HttpRequest, &mut HttpResponse) -> bool{};
// 指定方法后执行
svr.after = after(&HttpRequest, &mut HttpResponse) -> bool{};
// 指定回包header
svr.heder = BtreeMap;
// 启动服务
svr.start();
```
#### Config And Route  
have to config before server starts  
call lib_net::add_get_route or lib_net::add_post_route  


## WebsocketServer  
```rust
// 创建服务
let mut svr = WSServer::default();
// 设置服务地址
svr.addr = "url";
// 业务逻辑处理，返回值为None时发送pong
svr.handler = fn(&BytesMut) -> Option<Vec<u8>>;
// 指定超时时间
svr.with_timeout([Duration]);
// 启动服务
svr.start();
```
