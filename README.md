[TOC]
# lib_net

#### 介绍
网络相关


## 使用  

### HttpServer  

暂只支持单线程同步  
```rust
// 创建服务
HttpServer::default()
// 指定启动地址
.with_addr([url])
// 指定方法前执行
.with_before([before(&HttpRequest, &mut HttpResponse) -> bool ])
// 指定方法后执行
.with_after([after(&HttpRequest, &mut HttpResponse) -> bool ])
// 指定回包header
.with_header([BTreeMap])
// 启动服务
.start();
```
#### 配置及路由  
需在服务启动前设置  
路由暂只支持get和post，不支持path中携带参数，可调用lib_net::add_get_route或者lib_net::add_post_route  


## WebsocketServer  
```rust
// 创建服务
WSServer::default()
        // 设置服务地址
        .with_addr([url])
        // 业务逻辑处理，返回值为None时发送pong
        .with_handler([fn(&BytesMut) -> Option<Vec<u8>>])
        // 指定超时时间
        .with_timeout([Duration])
        // 启动服务
        .start();
```
