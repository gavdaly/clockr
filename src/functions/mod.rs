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
    axum::Extension, axum_session::Session, axum_session_sqlx::SessionPgPool, leptos::prelude::*,
    leptos_axum::extract, uuid::Uuid,
};

#[cfg(feature = "ssr")]
async fn current_user() -> Result<(Uuid, Extension<Session<SessionPgPool>>), ServerFnError> {
    use leptos::server_fn::error::NoCustomError;

    let session = match extract::<Extension<Session<SessionPgPool>>>().await {
        Ok(session) => session,
        Err(e) => {
            tracing::error!("Could not get session: {:?}", e);
            return Err(ServerFnError::<NoCustomError>::ServerError(
                "User not authenticated".into(),
            ));
        }
    };

    let Some(id) = session.get::<String>("id") else {
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "User not authenticated".into(),
        ));
    };

    let id = Uuid::parse_str(&id).map_err(|e| {
        ServerFnError::<NoCustomError>::ServerError(format!("Could not parse uuid: {}", e).into())
    })?;

    Ok((id, session))
}
