use axum::Json;
use serde::Deserialize;

// we deserialize the stringified json from browser to turn into json(the struct we defined)
#[derive(Deserialize, Debug)]
pub struct RequestUser {
    username: Option<String>, // an optional value
    password: String,
}

// take the json body and turn into a json of type request user
pub async fn validate_with_serde(Json(user): Json<RequestUser>) {
    // print a debug of the user object
    dbg!(user);
}
