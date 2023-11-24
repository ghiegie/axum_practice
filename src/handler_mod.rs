use axum::{
    extract::{Path, Query},
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HelloParams {
    name: Option<String>,
}

// /hello?name=sample_name (query param)
// query struct is used to extract queries from url
// Query<HelloParams> is a struct that will deserialize(convert) the query into helloparams struct
// the result of query<helloparams> is a tuple struct, so we can destructure it using Query(params)
// then use the params value
pub async fn hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {:?}", "HANDLER", params);

    // as_deref converts the Option<T> to Option<&T> or &Option<T>
    // unwrap_or returns the value of some or the given default val
    let name = params.name.as_deref().unwrap_or("World");
    name.to_owned()
}

// path is similar to query, but is used for getting values at
pub async fn hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {:?}", "HANDLER", name);
    name.to_owned()
}
