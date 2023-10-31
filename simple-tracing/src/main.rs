use std::net::SocketAddr;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tracing::info;
use tracing::instrument;

// This is the entry point of the program, the async main function.
#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber for logging.
    tracing_subscriber::fmt()
        //.json()  // This line is commented out; it's an option to format logs as JSON.
        .init();

    // Create an Axum router to define the HTTP routes and handlers.
    let routes = Router::new().route("/", get(handler));

    // Define the network address (IP and port) to bind the server to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    // Log that the server is starting and bind it to the specified address.
    info!(addr=?addr, "Connecting");

    // Create an Axum server and start serving the defined routes on the specified address.
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

// This is the handler for the root ("/") route.
#[instrument]
async fn handler() -> impl IntoResponse {
    // Log that someone has hit the root ("/") endpoint.
    info!(r#"Someone Hit /"#);

    // Return an HTML response ("<p>Henlo Warudoo</p>") for this route.
    Html("<p>Henlo Warudoo</p>")
}
