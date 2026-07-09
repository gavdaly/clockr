use crate::Result;
use leptos::prelude::*;

#[server]
pub async fn magic_sign_in(link: String) -> Result<()> {
    use crate::models::magic_link::MagicLink;
    use crate::models::passkey::user_has_passkey;
    use axum_session::SessionAnySession;
    use tracing::{error, info};

    let Some(session) = use_context::<SessionAnySession>() else {
        return Err(crate::Error::Unauthorized);
    };

    let Ok(magic_link) = MagicLink::consume(&link).await else {
        error!("COULD NOT GET USER FROM MAGIC LINK");
        return Err(crate::Error::Unauthorized);
    };

    info!(
        "Signed in user id: {} with {} magic link.",
        magic_link.user_id, magic_link.purpose
    );

    session.set_longterm(true);
    session.set("id", magic_link.user_id.to_string());

    if matches!(magic_link.purpose.as_str(), "invite" | "recovery")
        && !user_has_passkey(magic_link.user_id).await?
    {
        session.set("passkey_setup_required", true);
        leptos_axum::redirect("/app/passkeys/setup");
    } else {
        session.remove("passkey_setup_required");
        leptos_axum::redirect("/app");
    }

    Ok(())
}

#[server]
pub async fn create_magic_invite_link(user_id: String) -> Result<String> {
    use crate::models::magic_link::{MagicLink, MagicLinkPurpose};
    use chrono::Duration;
    use uuid::Uuid;

    let _ = super::current_admin_user_id().await?;

    let user_id = Uuid::parse_str(&user_id).map_err(|_| crate::Error::Unauthorized)?;
    let link_id = MagicLink::create_for(user_id, MagicLinkPurpose::Invite, Duration::days(14))
        .await
        .map_err(|_| crate::Error::InternalError)?;

    Ok(format!("{}/l/{}", app_base_url(), link_id))
}

#[server]
pub async fn create_magic_recovery_link(user_id: String) -> Result<String> {
    use crate::models::magic_link::{MagicLink, MagicLinkPurpose};
    use crate::models::user::UserDB;
    use chrono::Duration;
    use uuid::Uuid;

    let _ = super::current_admin_user_id().await?;

    let user_id = Uuid::parse_str(&user_id).map_err(|_| crate::Error::Unauthorized)?;
    let user = UserDB::get(user_id).await?;

    if user.email.is_none() {
        return Err(crate::Error::NotFound);
    }

    let link_id = MagicLink::create_for(user_id, MagicLinkPurpose::Recovery, Duration::minutes(30))
        .await
        .map_err(|_| crate::Error::InternalError)?;

    Ok(format!("{}/l/{}", app_base_url(), link_id))
}

#[cfg(feature = "ssr")]
fn app_base_url() -> String {
    std::env::var("APP_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string())
        .trim_end_matches('/')
        .to_string()
}
