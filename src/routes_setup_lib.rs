// dependencies from packages
use axum::{
    http::Method,
    middleware,
    routing::{get, post},
    Extension, Router,
};
use tower_http::cors::{Any, CorsLayer};

// dependencies from other files
use crate::routes_setup_lib::routes::*;

// submodule declaration
pub mod routes;

// for data sharing across routes
// implement clone to implement to each route
#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        // allow the declared methods
        .allow_methods([Method::GET, Method::POST])
        // allow request from any origin
        .allow_origin(Any);

    // create a shared data
    let shared_data = SharedData {
        message: "hello from shared data".to_owned(),
    };

    // routes are chosen not by order but by how close they are to specified path
    Router::new()
        .route("/hello", get(test))
        .route("/mirror_body_string", post(mirror_body_string))
        .route(
            "/mirror_body_json",
            post(mirror_body_json_lib::mirror_body_json),
        )
        .route("/path_var/15", get(hard_coded_path))
        .route("/path_var/:id", get(path_var))
        .route("/query_params", get(query_lib::query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_msg", get(middleware_msg))
        .layer(cors) // add cors as middleware fn
        .layer(Extension(shared_data)) // wrap shraed data inside an extension fore xtraction and response; then put inside layer
        .route(
            "/read_middleware_custom_header",
            get(custom_middleware_lib::read_middleware_custom_header),
        ) // does not get the layer from above
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/always_errors", get(always_errors))
        .route("/returns_201", post(returns_201))
        .route("/get_json", get(get_json_lib::get_json))
        .route(
            "/validate_data",
            post(validate_serde_lib::validate_with_serde),
        ) // should always handle error
}
