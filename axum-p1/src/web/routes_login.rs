use crate::{Error, Result};
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{info, warn};

pub fn route_login() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[derive(Deserialize, Serialize, Debug)]
struct LoginParams {
    username: String,
    password: String,
}

async fn api_login(Json(payload): Json<LoginParams>) -> Result<Json<Value>> {
    if payload.username != "akirami" || payload.password != "password" {
        warn!(?payload, "Login Failed");
        return Err(Error::LoginFail);
    }

    info!(?payload, "Login Succeeded");

    let body = Json(json!(payload));
    Ok(body)
}
