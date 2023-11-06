mod give_my_json;
mod give_my_string;
mod hello;
mod path_var;
mod query_params;

use axum::{
    routing::{get, post},
    Router,
};
use give_my_json::give_my_json;
use give_my_string::give_my_string;
use hello::hello_handler;
use path_var::{path_var, path_var_hardcoded};
use query_params::query_params;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_handler))
        .route("/give_my_string", post(give_my_string))
        .route("/give_my_json", post(give_my_json))
        .route("/path_var/:name", get(path_var))
        // when there is a get request to /path_var/Lucifer, only the below gets called not the above
        .route("/path_var/Lucifer", get(path_var_hardcoded))
        .route("/query_params", get(query_params))
}
