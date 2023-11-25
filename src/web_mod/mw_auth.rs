use async_trait::async_trait;
use axum::RequestPartsExt;
use axum::extract::{FromRequestParts, State};
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::{Cookies, Cookie};
use lazy_regex::regex_captures;

use crate::ctx_mod::Ctx;
use crate::model_mod::ModelController;
use crate::web_mod::AUTH_TOKEN;
use crate::error_mod::*;

pub async fn mw_require_auth<B>(ctx: Result<Ctx>, req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver<B>(_mc: State<ModelController>, cookies: Cookies, mut req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token.ok_or(Error::AuthFailNoAuthTokenCookie).and_then(parse_token) {
        Ok((user_id, _exp, _sign)) => {
            Ok(Ctx::new(user_id))
        },
        Err(e) => Err(e),
    };

    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    req.extensions_mut().insert(result_ctx);
    
    Ok(next.run(req).await)
}

/// parse a token of format `user-[user-id].[expiration].signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthFailTokenWrongFormat)?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    // Parts is a request parts of body reqeuest
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        let d = parts.extensions.get::<Result<Ctx>>().ok_or(Error::AuthFailCtxNotInRequestExt)?.clone();

        d
    }
}