use crate::models::user::UserDisplay;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use tracing::{error, info};

/// Retrieves the current user from the session, if one exists
///
/// # Returns
/// - `Ok(Some(UserDisplay))` if a user is found and authenticated
/// - `Ok(None)` if no user is authenticated
/// - `Err(ServerFnError)` if there are session or database errors
#[server]
#[tracing::instrument]
pub async fn get_curent_user() -> Result<Option<UserDisplay>, ServerFnError> {
    use uuid::Uuid;

    let Some(session) = use_context::<UserSession>() else {
        error!("Error getting session");
        return Err(ServerFnError::ServerError("Error Finding Session".into()));
    };

    info!("Session: {:?}", session.0);

    let Some(id) = session.0.get("id") else {
        info!("User not Authenticated");
        return Ok(None);
    };

    let Ok(user) = UserDisplay::get(id).await else {
        error!("Could not find User for session");
        return Err(ServerFnError::ServerError("Could Not Find User.".into()));
    };

    Ok(Some(user))
}

/// Retrieves and verifies the currently authenticated user
///
/// # Returns
/// - `Ok(UserDisplay)` if a valid user is found
/// - `Err(ServerFnError)` if session is missing, user is not authenticated, or user cannot be found
#[server]
#[tracing::instrument]
async fn check_in() -> Result<UserDisplay, ServerFnError> {
    // use crate::models::sessions::{close_session, get_open_session, new_session};
    use axum_session::SessionAnySession;
    use leptos::prelude::server_fn::error::*;
    use uuid::Uuid;

    let Some(session) = use_context::<SessionAnySession>() else {
        error!("Cound not load session context");
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "Session missing.".into(),
        ));
    };

    let Some(id) = session.get::<Uuid>("id") else {
        info!("User not Authenticated");
        leptos_axum::redirect("/");
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "Error getting Session!".into(),
        ));
    };

    let Ok(user) = crate::models::user::UserDisplay::get(id).await else {
        error!("Could not find User");
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "Could Not Find User.".into(),
        ));
    };

    Ok(user)
}
