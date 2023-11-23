use std::net::SocketAddr;
use axum::Router;

pub async fn run_server(addr: SocketAddr, router: Router) {
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();    
}