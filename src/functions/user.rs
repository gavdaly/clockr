use crate::models::CurrentUser;
use leptos::{prelude::*, server_fn::error::NoCustomError};

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
    use crate::models::UserTodayDB;

    trace!("Getting Current User");

    let (id, session) = super::current_user()
        .await?;

    let Ok(user) = UserTodayDB::get(&id.to_string()).await else {
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

    let (id, _) = super::current_user().await?;

    TimeLogDB::add(id).await?;

    Ok(())
}
