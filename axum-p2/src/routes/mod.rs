mod give_my_json;
mod give_my_string;
mod hello;

use axum::{
    routing::{get, post},
    Router,
};
use give_my_json::give_my_json;
use give_my_string::give_my_string;
use hello::hello_handler;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_handler))
        .route("/give_my_string", post(give_my_string))
        .route("/give_my_json", post(give_my_json))
}
