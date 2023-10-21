use crate::{ error::{ Error, Result }, routes::{ v1::auth::error::AuthError, AUTH_TOKEN } };
use axum::{ http::Request, middleware::Next, response::Response };
use tower_cookies::Cookies;
use tracing::debug;

pub async fn with_auth<B>(cookies: Cookies, req: Request<B>, next: Next<B>) -> Result<Response> {
    debug!("{:<12} - {}", "MIDDLEWARE", "with_auth");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    auth_token.ok_or(Error::AuthError(AuthError::InvalidToken))?;
    Ok(next.run(req).await)
}
