use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: String::from("sample json string"),
        count: 123,
        username: String::from("sample json id"),
    };

    Json(data)
}
