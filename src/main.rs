use std::net::SocketAddr;
use axum::{routing::get, Router};

use axum_practice::fn_mod::*;
use axum_practice::handler_mod::*;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(hello));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("->> LISTENING on {}\n", addr);

    run_server(addr, routes_hello).await;
}