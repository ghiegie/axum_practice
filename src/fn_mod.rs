use axum::{response::{Response, IntoResponse}, Router, Json, http::{Uri, Method}};
use serde_json::json;
use std::net::SocketAddr;
use uuid::Uuid;

use crate::{error_mod::Error, ctx_mod::Ctx, log_mod::log_request};

pub async fn run_server(addr: SocketAddr, router: Router) {
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

// it takes the response from a route, can return the same response or different response
pub async fn main_response_mapper(res: Response, ctx: Option<Ctx>, uri: Uri, req_method: Method) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error.as_ref().map(|(status_code, client_error)| {
        let client_error_body = json!({
            "error": {
                "type": client_error.as_ref(),
                "req_uuid": uuid.to_string(),
            }
        });

        println!("  ->> client_error_body: {client_error_body}");
        (*status_code, Json(client_error_body)).into_response()
    });

    let client_error = client_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    error_response.unwrap_or(res)
}
