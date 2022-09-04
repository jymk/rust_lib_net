[TOC]
# lib_net

#### Description
about net


## Use  

### HttpServer  

only support single thread and sync now  
```rust
// create server
HttpServer::default()
// set address
.with_addr([url])
// set function before login
.with_before([before(&HttpRequest, &mut HttpResponse) -> bool ])
// set function after login
.with_after([after(&HttpRequest, &mut HttpResponse) -> bool ])
// set response header
.with_header([BTreeMap])
// start server
.start();
```
#### Config And Route  
have to config before server starts  
route only support get and post method, not support path with param，call lib_net::add_get_route or lib_net::add_post_route  


## WebsocketServer  
```rust
// create server
WSServer::default()
        // set address
        .with_addr([url])
        // logic function, send pong when returns None
        .with_handler([fn(&BytesMut) -> Option<Vec<u8>>])
        // set timeout
        .with_timeout([Duration])
        // start server
        .start();
```
