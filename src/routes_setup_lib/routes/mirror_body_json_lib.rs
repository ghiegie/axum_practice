use axum::Json;
use serde::{Deserialize, Serialize};

// the structure of the response we get
#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJSON {
    message: String,
}

// another structure of data to receive
#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJSONResponse {
    message: String,
    message_from_server: String,
}

// take the body of request, and "turn it into something" that has the structure of MirrorJSON
// then return it into a json with a structure of MirrorJSONResponse
pub async fn mirror_body_json(Json(body): Json<MirrorJSON>) -> Json<MirrorJSONResponse> {
    // create the structure of json
    // then convert it into json(?)
    Json(MirrorJSONResponse {
        message: body.message,
        message_from_server: String::from("Hello from Axum"),
    })
}
