use std::collections::HashMap;

use axum::{extract::Query, Json};

pub async fn query_params(
    Query(params): Query<HashMap<String, String>>,
) -> Json<HashMap<String, String>> {
    Json(params)
}
