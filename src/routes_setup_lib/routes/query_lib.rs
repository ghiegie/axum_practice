// dependencies from packages
use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

// dependencies from other files

// submodule declaration

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    message: String,
    id: i32,
}

// take the query in the url convert it into Query of type QueryParams
// then return it in json of type queryparams
pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    // return json of query structure
    Json(query)
}
