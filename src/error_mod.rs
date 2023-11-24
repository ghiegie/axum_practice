use axum::{http::StatusCode, response::*};

#[derive(Debug)]
pub enum Error {
    LoginFail,
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    TicketDeleteFailIdNotFound { id: u64 },
}

// alias result type of this form Result<T(any val), Error(w/c is the error enum we created)
pub type Result<T> = core::result::Result<T, Error>;

impl IntoResponse for Error {
    // take the Error enum we created and return Response struct
    fn into_response(self) -> Response {
        println!("->> {:<12} - {:?}", "INTO_RES", self);

        // return a tuple of statuscode enum variant and a &str
        // then turn the tuple it into response
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
