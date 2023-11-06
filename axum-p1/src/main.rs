use std::{collections::HashMap, net::SocketAddr};

use axum::{
    extract::{Path, Query},
    http::{Method, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Json, Router,
};
use ctx::Ctx;
use serde_json::json;
use uuid::Uuid;

mod error;
use crate::{log::log_request, models::ModelController, web::mw_auth::mw_ctx_resolver};

pub use self::error::{Error, Result};

pub mod models;

mod ctx;
mod log;
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

    let route_apis = web::routes_crud::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    // Create an Axum router for defining HTTP routes and their handlers.
    let routes = Router::new()
        .merge(routes_hello())
        .merge(route_login())
        .nest_service("/api", route_apis)
        // adding the middleware / layer
        .layer(middleware::map_response(main_respone_mapper))
        // this middle takes the cookies from the previous middleware for resolving ctx
        .layer(middleware::from_fn_with_state(mc.clone(), mw_ctx_resolver))
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
async fn main_respone_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    // uncomment to check how the http reponse looks
    // println!("{res:#?}");
    // println!();
    let uuid = Uuid::new_v4();

    // -- Get eventual response error
    let service_error = res.extensions().get::<Error>();
    let client_error = service_error.map(|se| se.client_status_and_error());

    // -- If Client error, build new response
    let error_response = client_error.as_ref().map(|(statuscode, client_error)| {
        let client_err_body = json!({
            "error":{
                "type":client_error.as_ref(),
                "req_uuid":uuid.to_string(),
            }
        });
        info!(?client_err_body, "Client Error Body");
        println!("{client_err_body:?}");
        // we are using *statuscode because, statuscode is referenced, hence we need to deref it
        (*statuscode, Json(client_err_body)).into_response()
        // the below still works because axum::StatusCode implements Clone, hence when we deref it,
        // it gets clones and thus we can still use the var statuscode
        // println!("{statuscode:?}");
    });
    // info!("Server log line - {uuid} - Error: {service_error:?}");
    let client_error = client_error.unzip().1;
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    error_response.unwrap_or(res)
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
