use axum::{http::StatusCode, response::*};
use serde::Serialize;

#[derive(Clone, Debug, strum_macros::AsRefStr, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
    TicketDeleteFailIdNotFound { id: u64 },
}

// alias result type of this form Result<T(any val), Error(w/c is the error enum we created)
pub type Result<T> = core::result::Result<T, Error>;

impl IntoResponse for Error {
    // take the Error enum we created and return Response struct
    fn into_response(self) -> Response {
        println!("->> {:<12} - {:?}", "INTO_RES", self);

        // create a placeholder axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // insert the error into the response
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            Self::AuthFailNoAuthTokenCookie | Self::AuthFailTokenWrongFormat | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            Self::TicketDeleteFailIdNotFound { .. } => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR)
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
