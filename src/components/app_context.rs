use std::ops::Deref;

use crate::models::user::UserDisplay;
use leptos::prelude::*;

#[derive(Clone)]
pub struct AppContext {
    pub user: Option<UserDisplay>,
}

pub fn create_app_context() -> AppContext {
    let user_fetch = LocalResource::new(get_curent_user);
    let user_fetch = user_fetch.read();

    let Some(user_fetch) = user_fetch.deref() else {
        return AppContext { user: None };
    };

    let Ok(user) = &**user_fetch else {
        return AppContext { user: None };
    };

    AppContext { user: user.clone() }
}

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
async fn check_in(_latitude: f64, _longitude: f64, _accuracy: f64) -> Result<(), ServerFnError> {
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

    // match is_close(latitude, longitude, accuracy).await {
    //     Ok(_) => (),
    //     Err(e) => return Err(e),
    // };

    // check for existing session
    match get_open_session(&id).await {
        Ok(sess) => {
            // if no session create new session
            let _ = close_session(&sess.id).await;
        }
        Err(_) => {
            // else close exsiting session
            let _ = new_session(&id).await;
        }
    };

    leptos_axum::redirect("/app");

    Ok(())
}
