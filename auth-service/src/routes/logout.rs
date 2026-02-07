use axum::response::IntoResponse;
use axum::http::status::StatusCode;
use axum_extra::extract::CookieJar;

use crate::{
    domain::AuthAPIError,
    utils::constants::JWT_COOKIE_NAME,
    utils::auth};

pub async fn logout(jar: CookieJar) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    // Retrieve JWT cookie from the `CookieJar`
    let cookie = jar.get(&JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;

    let token = cookie.value().to_owned();

    // validate_token
    let claims = auth::validate_token(&token).await
        .map_err(|_| AuthAPIError::InvalidToken)?;
    
    Ok((jar, StatusCode::OK.into_response()))
}