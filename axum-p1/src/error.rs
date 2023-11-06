use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::error;

pub type Result<T> = core::result::Result<T, Error>;

//Key Idea here: NEVER send internal details about the error to the client

#[derive(Clone, Debug, strum_macros::AsRefStr, Serialize)]
#[serde(tag = "type", content = "data")] // type if the variant name and data is data inside variant like "id"
pub enum Error {
    LoginFail,
    AuthFailedNoToken,
    AuthFailedWrongTokenFormat,
    AuthFailCtxNotInRequest,
    NoTicketID { id: usize },
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

#[allow(unreachable_patterns)]
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            // -- Login
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // -- Auth
            Self::AuthFailCtxNotInRequest
            | Self::AuthFailedNoToken
            | Self::AuthFailedWrongTokenFormat => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Model
            Self::NoTicketID { .. } => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),

            // -- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!(Error=?self);
        // (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
        // Create a placeholder Axum Response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into Response
        response.extensions_mut().insert(self);

        // Finally return response
        response
    }
}
