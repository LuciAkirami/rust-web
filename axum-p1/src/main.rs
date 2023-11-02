use std::{collections::HashMap, net::SocketAddr};

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};

mod error;
use crate::models::ModelController;

pub use self::error::{Error, Result};

pub mod models;

mod web;
use web::routes_login::route_login;

use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use tracing::info;
use tracing::instrument;

// The entry point of the program, an async main function.
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber for logging.
    tracing_subscriber::fmt()
        //.json()  // This line is commented out; it's an option to format logs as JSON.
        .init();

    let mc = ModelController::new().await?;

    // Create an Axum router for defining HTTP routes and their handlers.
    let routes = Router::new()
        .merge(routes_hello())
        .merge(route_login())
        .nest_service("/api", web::routes_crud::routes(mc.clone()))
        // adding the middleware / layer
        .layer(middleware::map_response(main_respone_mapper))
        // adding cookie middleware
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // Define the network address (IP and port) to bind the server to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    info!(addr=?addr,"Connecting to");
    // Create an Axum server and start serving the defined routes on the specified address.
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handle_request))
        .route("/hello/:name", get(handle_request2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// tower middleware, this middleware will take the Reponse and do something
async fn main_respone_mapper(res: Response) -> Response {
    println!();
    // uncomment to check how the http reponse looks
    // println!("{res:#?}");
    // println!();
    res
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
