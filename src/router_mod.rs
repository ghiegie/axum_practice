use axum::{routing::get, Router};

use crate::handler_mod::*;

// creates a router holding handlers
pub fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/hello2/:name", get(hello2)) // for getting values from path
}
