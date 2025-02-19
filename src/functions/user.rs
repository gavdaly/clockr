use crate::models::user::UserToday;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use tracing::{error, info, trace};

/// Retrieves the current user from the session, if one exists
///
/// # Returns
/// - `Ok(Some(UserToday))` if a user is found and authenticated
/// - `Ok(None)` if no user is authenticated
/// - `Err(ServerFnError)` if there are session or database errors
#[server]
#[tracing::instrument]
pub async fn get_current_user() -> Result<Option<UserToday>, ServerFnError> {
    use axum::extract::Extension;
    use axum_session::Session;
    use axum_session_sqlx::SessionPgPool;
    use leptos_axum::extract;
    use uuid::Uuid;

    let session: Extension<Session<SessionPgPool>> = extract().await?;
    info!("Session: {:?}", session);

    let Some(id) = session.get("id") else {
        trace!("User not Authenticated, Could not find ID: {:?}", session);
        return Ok(None);
    };

    let Ok(user) = UserToday::get(id).await else {
        error!("Could not find User for session");
        return Err(ServerFnError::ServerError("Could Not Find User.".into()));
    };

    Ok(Some(user))
}

/// Retrieves and verifies the currently authenticated user
///
/// # Returns
/// - `Ok(UserToday)` if a valid user is found
/// - `Err(ServerFnError)` if session is missing, user is not authenticated, or user cannot be found
#[server]
#[tracing::instrument]
async fn check_in() -> Result<UserToday, ServerFnError> {
    use crate::models::sessions::{close_session, get_open_session, new_session};
    use axum::extract::Extension;
    use axum_session::Session;
    use axum_session_sqlx::SessionPgPool;
    use leptos_axum::extract;
    use uuid::Uuid;

    let session: Extension<Session<SessionPgPool>> = extract().await?;
    info!("Session: {:?}", session);

    let Some(id) = session.get("id") else {
        trace!("User not Authenticated, Could not find ID: {:?}", session);
        return Err(ServerFnError::ServerError("User not authenticated".into()));
    };

    let Ok(user) = UserToday::get(id).await else {
        error!("Could not find User for session");
        return Err(ServerFnError::ServerError("Could Not Find User.".into()));
    };

    match get_open_session(&user.id).await {
        Ok(user_session) => {
            close_session(&user_session.id).await?;
        }
        _ => {
            new_session(&user.id).await?;
        }
    };

    let Ok(user) = UserToday::get(id).await else {
        error!("Could not find User for session");
        return Err(ServerFnError::ServerError("Could Not Find User.".into()));
    };
    Ok(user)
}
