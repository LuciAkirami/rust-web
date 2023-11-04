use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;

pub type Result<T> = core::result::Result<T, Error>;

//Key Idea here: NEVER send internal details about the error to the client

#[derive(Debug)]
pub enum Error {
    LoginFail,
    AuthFailedNoToken,
    AuthFailedWrongTokenFormat,
    NoTicketID { id: usize },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!(Error=?self);
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
