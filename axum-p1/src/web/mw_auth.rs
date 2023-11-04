use crate::error::{Error, Result};
use axum::{http::Request, middleware::Next, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    let (_user_id, _exp, _sign) = auth_token
        .ok_or(Error::AuthFailedNoToken)
        .and_then(token_parser)?;

    //auth_token.ok_or(Error::AuthenticanFailed)?;
    Ok(next.run(req).await)
}

fn token_parser(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailedWrongTokenFormat)?;
    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailedWrongTokenFormat)?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}
