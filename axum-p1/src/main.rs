use std::net::SocketAddr;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

// The entry point of the program, an async main function.
#[tokio::main]
async fn main() {
    // Create an Axum router for defining HTTP routes and their handlers.
    let routes = Router::new().route("/hello", get(handle_request));

    // Define the network address (IP and port) to bind the server to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // Create an Axum server and start serving the defined routes on the specified address.
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

async fn handle_request() -> impl IntoResponse {
    Html("<p>Henlo Warudoo</p>")
}
