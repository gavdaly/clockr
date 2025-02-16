use crate::models::user::UserDisplay;
use leptos::prelude::*;

#[server]
pub async fn get_curent_user() -> Result<Option<UserDisplay>, ServerFnError> {
    use axum_session::SessionAnySession;
    use leptos::prelude::server_fn::error::*;
    use uuid::Uuid;

    let Some(session) = use_context::<SessionAnySession>() else {
        // leptos::tracing::error!("| * Error getting settion");
        return Err(ServerFnError::ServerError(
            "Error Finding Session 30".into(),
        ));
    };

    let Some(id) = session.get::<Uuid>("id") else {
        // leptos::tracing::info!("| * User not signed in");
        return Ok(None);
    };

    let Ok(user) = UserDisplay::get(id).await else {
        // leptos::tracing::error!("| * Could not find User for session");
        return Err(ServerFnError::ServerError("Could Not Find User.".into()));
    };

    Ok(Some(user))
}

#[server]
async fn check_in() -> Result<UserDisplay, ServerFnError> {
    use crate::models::sessions::{close_session, get_open_session, new_session};
    use leptos::prelude::server_fn::error::*;
    use uuid::Uuid;
    // Get User
    use axum_session::SessionAnySession;
    let session = use_context::<SessionAnySession>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Session missing.".into()))?;
    let id = session.get::<Uuid>("id").ok_or_else(|| {
        ServerFnError::<NoCustomError>::ServerError("Error getting Session!".into())
    })?;

    leptos_axum::redirect("/app");

    let user = crate::models::user::UserDisplay::get(id)
        .await
        .map_err(|_| ServerFnError::<NoCustomError>::ServerError("Could Not Find User.".into()))?;

    Ok(user)
}
