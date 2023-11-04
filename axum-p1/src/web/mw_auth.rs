use crate::ctx::Ctx;
use crate::error::{Error, Result};
use crate::models::ModelController;
use axum::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::{http::Request, middleware::Next, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use tracing::info;

use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    info!(?ctx, "Middleware - Require Auth");
    ctx?;
    //auth_token.ok_or(Error::AuthenticanFailed)?;
    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver<B>(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    info!("Middleware - CTX Resolver");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result = match auth_token
        .ok_or(Error::AuthFailedNoToken)
        .and_then(token_parser)
    {
        Ok((user_id, _exp, _sign)) => Ok(Ctx::new(user_id)),
        Err(e) => Err(e),
    };

    if result.is_err() && !matches!(result, Err(Error::AuthFailedNoToken)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    req.extensions_mut().insert(result);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        info!("Extractor - Ctx");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequest)?
            .clone()
    }
}

fn token_parser(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailedWrongTokenFormat)?;
    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailedWrongTokenFormat)?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}
