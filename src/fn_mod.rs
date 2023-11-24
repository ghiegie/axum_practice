use axum::{response::Response, Router};
use std::net::SocketAddr;

pub async fn run_server(addr: SocketAddr, router: Router) {
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

// it takes the response from a route, can return the same response or different response
pub async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}
