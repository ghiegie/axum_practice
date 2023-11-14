// dependencies from packages
use axum::{
    extract::{Path, TypedHeader},
    headers::UserAgent,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};

// dependencies from other files
use super::SharedData;
use crate::routes_setup_lib::custom_middleware_lib::HeaderMessage;

// submodule declaration
pub mod custom_middleware_lib;
pub mod get_json_lib;
pub mod mirror_body_json_lib;
pub mod query_lib;
pub mod validate_serde_lib;

pub async fn test() -> String {
    String::from("Test")
}

// extract string in a body of request
pub async fn mirror_body_string(body: String) -> String {
    body
}

// extract the value from path
// path/:id
pub async fn path_var(Path(id): Path<i32>) -> String {
    id.to_string()
}

// hard coded
// path 15
// this will be chosen because it is more close
pub async fn hard_coded_path() -> String {
    format!("You got {}", 15)
}

// get and return the typed header
pub async fn mirror_user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}

// get the custom header
pub async fn mirror_custom_header(headers: HeaderMap) -> String {
    // headers is a struct; extract the value from its "key", unwrap for err
    // convert to &str, unwrap for err
    // convert to String
    headers
        .get("User-Agent")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}

// extract shared data, convert to extension of type shared data
// return to string
pub async fn middleware_msg(Extension(shared_data): Extension<SharedData>) -> String {
    // access the msg inside shared_data
    shared_data.message
}

pub async fn set_middleware_custom_header<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message
        .to_str()
        .map_err(|_error| StatusCode::BAD_REQUEST)?
        .to_owned();
    let extensions = request.extensions_mut();
    extensions.insert(HeaderMessage(message.to_owned()));
    Ok(next.run(request).await)
}

// for handling errors
pub async fn always_errors() -> Result<(), StatusCode> {
    Err(StatusCode::IM_A_TEAPOT)
}

pub async fn returns_201() -> Response {
    (StatusCode::CREATED, "This is a 201".to_owned()).into_response()
}
