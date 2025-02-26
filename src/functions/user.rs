use crate::models::user::CurrentUser;
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
pub async fn get_current_user() -> Result<CurrentUser, ServerFnError> {
    use axum::extract::Extension;
    use axum_session::Session;
    use axum_session_sqlx::SessionPgPool;
    use leptos::prelude::server_fn::error::*;
    use leptos_axum::extract;
    use crate::models::UserDB;

    trace!("Getting Current User");

    let session: Extension<Session<SessionPgPool>> = extract().await.map_err(|e| {
        error!("Could not get session: {:?}", e);
        ServerFnError::<NoCustomError>::ServerError(e.to_string())
    })?;
    
    info!("Session: {:?}", session);

    let Some(id) = session.get("id") else {
        trace!("User not Authenticated, Could not find ID: {:?}", session);
        session.clear();
        return Ok(CurrentUser::Guest);
    };

    let Ok(user) = UserDB::get(id).await else {
        error!("Could not find User for session");
        session.clear();
        return Ok(CurrentUser::Guest);
    };


    Ok(CurrentUser::Authenticated(user.into()))
}

/// Retrieves and verifies the currently authenticated user
///
/// # Returns
/// - `Ok(UserToday)` if a valid user is found
/// - `Err(ServerFnError)` if session is missing, user is not authenticated, or user cannot be found
#[server]
#[tracing::instrument]
async fn check_in() -> Result<(), ServerFnError> {
    use crate::models::TimeLogDB;
    use axum::extract::Extension;
    use axum_session::Session;
    use leptos::prelude::server_fn::error::*;
    use axum_session_sqlx::SessionPgPool;
    use leptos_axum::extract;
    use uuid::Uuid;

    let session: Extension<Session<SessionPgPool>> = extract().await?;
    info!("Session: {:?}", session);

    let Some(id) = session.get::<String>("id") else {
        trace!("User not Authenticated, Could not find ID: {:?}", session);
        return Err(ServerFnError::<NoCustomError>::ServerError("User not authenticated".into()));
    };

    let id = Uuid::parse_str(&id).expect("Should be valid uuid");

    TimeLogDB::add(id).await?;

    Ok(())
}
