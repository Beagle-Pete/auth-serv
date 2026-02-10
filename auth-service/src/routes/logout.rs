use axum::{response::IntoResponse,
    http::status::StatusCode,
    extract::State,
};
use axum_extra::extract::CookieJar;


use crate::{
    domain::AuthAPIError,
    utils::constants::JWT_COOKIE_NAME,
    utils::auth,
    AppState,
};

pub async fn logout(State(state): State<AppState>, jar: CookieJar) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    // Retrieve JWT cookie from the `CookieJar`
    let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;

    let token = cookie.value().to_owned();

    // validate_token
    let _claims = auth::validate_token(&token).await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    let mut banned_token_store = state.banned_token_store.write().await;
    banned_token_store.add_token(&token)
        .await
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    let jar = jar.remove(JWT_COOKIE_NAME);
    
    Ok((jar, StatusCode::OK.into_response()))
}