use crate::{Error, Result};
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::{info, warn};

pub fn route_login() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[derive(Deserialize, Serialize, Debug)]
struct LoginParams {
    username: String,
    password: String,
}

async fn api_login(cookies: Cookies, Json(payload): Json<LoginParams>) -> Result<Json<Value>> {
    if payload.username != "akirami" || payload.password != "password" {
        warn!(?payload, "Login Failed");
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new("auth-token", "user-1.exp.sign"));

    info!(?payload, "Login Succeeded");

    let body = Json(json!(payload));
    Ok(body)
}
