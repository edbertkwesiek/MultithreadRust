
#[allow(unused)]
fn main() {
extern crate async_std;
use async_std::{
    prelude::*, // 1
    task, // 2
    net::{TcpListener, ToSocketAddrs}, // 3
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>; // 4
}



#![allow(unused)]
fn main() {
extern crate async_std;
use async_std::{
    net::{TcpListener, ToSocketAddrs},
    prelude::*,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> { // 1

    let listener = TcpListener::bind(addr).await?; // 2
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await { // 3
        // TODO
    }
    Ok(())
}
}


#[allow(unused)]
fn main() {
extern crate async_std;
use async_std::{
    net::{TcpListener, ToSocketAddrs},
    prelude::*,
    task,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> { // 1
    let listener = TcpListener::bind(addr).await?; // 2
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await { // 3
        // TODO
    }
    Ok(())
}

// main
fn run() -> Result<()> {
    let fut = accept_loop("127.0.0.1:8080");
    task::block_on(fut)
}
}
