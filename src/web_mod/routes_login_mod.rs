use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::error_mod::*;
use crate::web_mod::*;

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

// json is a body extractor
// u can only have one body extractor per route
// json is a struct used to convert body into type LoginPayload
// return our custom result that has the value type(inported from serde)
async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: implement real db/auth logic
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // FIXME: Implement real auth-token generation/signature
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    // create success body
    // create Json tuple struct
    // json! constructs a serd::value stuct from given json literal
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}
