#![feature(ip)]
///! A standalone http server which simply reports back the IP address of the caller,
///! discarding URL and request info.
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::stream::StreamExt;
use futures_lite::io::AsyncWriteExt;
use std::fmt::Display;
use std::net::SocketAddr;
use std::alloc::System;
use std::time::SystemTime;

// Reduce binary size
#[global_allocator]
static A: System = System;

const HEAD: &str = "HTTP/1.1 200 OK\r\ncontent-type: text/plain;charset=utf-8\r\ncache-control: no-cache,no-store\r\ncontent-length: ";

#[async_std::main]
async fn main() {
    log("Startup");
    let listener = TcpListener::bind("::0:8090").await.unwrap();
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.find(|_x| true).await {
        match stream {
            Ok(str) => handle_connection(str).await,
            Err(e) => log_err(&e),
        }
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut address = None;
    let contents = match stream.peer_addr() {
        Ok(addr) => {
            address = Some(addr);
            let canon = addr.ip().to_canonical();
            log(canon);
            canon.to_string()
        }
        Err(e) => {
            log_err(&e);
            format!("{e}")
        }
    };
    let length = contents.bytes().len().to_string();
    let response = format!("{HEAD}{length}\r\n\r\n{contents}");
    match stream.write_all(response.as_bytes()).await {
        Ok(_) => match stream.flush().await {
            Ok(_) => (),
            Err(e) => {
                log_net_err(address, &e);
            }
        },
        Err(e) => {
            log_net_err(address, &e);
        }
    }
}

// Just dirt simple stdout logging
fn log_err<T: std::error::Error>(e: &T) {
    let when = now();
    println!("{}\t{}", when, e);
}

fn log_net_err<T: std::error::Error>(addr: Option<SocketAddr>, e: &T) {
    let when = now();
    let astring = addr.map(|a| a.to_string()).unwrap_or("--".to_string());
    println!("{} {}:\t{}", when, astring, e);
}

fn log<T : Display>(what : T) {
    println!("{}\t{what}", now());
}

fn now() -> String {
    // Just log the unix timestamp, rather than depend on chrono or time
    format!("{:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())
}
