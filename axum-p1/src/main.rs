use std::net::SocketAddr;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
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
    let routes = Router::new().route("/hello", get(handle_request));

    // Define the network address (IP and port) to bind the server to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    info!(addr=?addr,"Connecting to");
    // Create an Axum server and start serving the defined routes on the specified address.
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

#[instrument]
async fn handle_request() -> impl IntoResponse {
    // Log that client has hit the "/hello" route
    info!("Handling Hello");
    Html("<p>Henlo Warudoo</p>")
}
