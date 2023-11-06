mod hello;
use hello::hello_handler;

use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new().route("/", get(hello_handler))
}
