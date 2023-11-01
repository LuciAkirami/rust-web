use std::{collections::HashMap, net::SocketAddr};

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};

mod error;
pub use self::error::{Error, Result};

mod web;
use web::routes_login::route_login;

use tower_http::services::ServeDir;
use tracing::info;
use tracing::instrument;

// The entry point of the program, an async main function.
#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber for logging.
    tracing_subscriber::fmt()
        //.json()  // This line is commented out; it's an option to format logs as JSON.
        .init();

    // Create an Axum router for defining HTTP routes and their handlers.
    let routes = Router::new()
        .merge(routes_hello())
        .merge(route_login())
        .fallback_service(routes_static());

    // Define the network address (IP and port) to bind the server to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    info!(addr=?addr,"Connecting to");
    // Create an Axum server and start serving the defined routes on the specified address.
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handle_request))
        .route("/hello/:name", get(handle_request2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
#[instrument]
async fn handle_request(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    // Log that client has hit the "/hello" route
    info!("Handling Hello");
    println!("Parameters: {:?}", params);
    Html("<p>Henlo Warudoo</p>")
}

#[instrument]
async fn handle_request2(Path(name): Path<String>) -> impl IntoResponse {
    // Log that client has hit the "/hello" route
    info!("Handling Hello with name in Path");
    Html(format!("<p>Henlo {name}</p>"))
}
