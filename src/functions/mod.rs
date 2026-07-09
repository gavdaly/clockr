pub mod add_time;
pub mod delete_time;
pub mod magic_link_sign_in;
pub mod passkeys;
pub mod user;

pub use add_time::*;
pub use delete_time::*;
pub use magic_link_sign_in::*;
pub use passkeys::*;
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
pub(crate) async fn current_user() -> Option<(Uuid, Extension<Session<SessionPgPool>>)> {
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

#[cfg(feature = "ssr")]
pub(crate) async fn current_admin_user_id() -> crate::Result<Uuid> {
    let (id, _) = current_user().await.ok_or(crate::Error::Unauthorized)?;
    let user = crate::models::user::UserDB::get(id).await?;

    if user.state != 1 {
        return Err(crate::Error::Unauthorized);
    }

    Ok(id)
}
