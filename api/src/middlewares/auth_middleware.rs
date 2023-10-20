use crate::{
    error::{auth::AuthError, Result},
    routers::AUTH_TOKEN,
    Error,
};
use axum::{http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

pub async fn with_auth<B>(cookies: Cookies, req: Request<B>, next: Next<B>) -> Result<Response> {
    print!("->> {:<12} - {}", "AUTH", "with_auth");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    auth_token.ok_or(Error::AuthError(AuthError::InvalidToken))?;
    Ok(next.run(req).await)
}
