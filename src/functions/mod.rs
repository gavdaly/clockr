pub mod add_time;
pub mod delete_time;
pub mod magic_link_sign_in;
pub mod user;

pub use add_time::*;
pub use delete_time::*;
pub use magic_link_sign_in::*;
pub use user::*;

#[cfg(feature = "ssr")]
use {
    axum::Extension,
    axum_session::Session,
    axum_session_sqlx::SessionPgPool,
    leptos_axum::extract,
    tracing::{error, trace},
    uuid::Uuid,
};

#[cfg(feature = "ssr")]
async fn current_user() -> Option<(Uuid, Extension<Session<SessionPgPool>>)> {

    let session = match extract::<Extension<Session<SessionPgPool>>>().await {
        Ok(s) => s,
        Err(e) => {
            trace!("Could not get session: {:?}", e);
            return None;
        }
    };

    let Some(id) = session.get::<String>("id") else {
        trace!("Could not get id from session: {session:?}");
        return None;
    };

    let Ok(id) = Uuid::parse_str(&id) else {
        error!("Invalid UUID: {id}");
        return None;
    };

    Some((id, session))
}

