use crate::Result;
use leptos::prelude::*;

#[server]
pub async fn magic_sign_in(link: String) -> Result<()> {
    use crate::models::magic_link::MagicLink;
    use crate::models::passkey::user_has_passkey;
    use crate::models::user::mark_user_email_verified;
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

    if matches!(magic_link.purpose.as_str(), "invite" | "recovery") {
        if let Some(email) = magic_link.email.as_deref() {
            mark_user_email_verified(magic_link.user_id, email).await?;
        }
    }

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
    use tracing::info;
    use uuid::Uuid;

    let admin_user_id = super::current_admin_user_id().await?;

    let user_id = Uuid::parse_str(&user_id).map_err(|_| crate::Error::Unauthorized)?;
    let user = crate::models::user::UserDB::get(user_id).await?;
    let link_id = MagicLink::create_for_email(
        user_id,
        MagicLinkPurpose::Invite,
        Duration::days(14),
        user.email.as_deref(),
    )
    .await
    .map_err(|_| crate::Error::InternalError)?;

    info!(
        admin_user_id = %admin_user_id,
        target_user_id = %user_id,
        link_id = %link_id,
        purpose = "invite",
        "Admin created magic link."
    );

    Ok(format!("{}/l/{}", app_base_url(), link_id))
}

#[server]
pub async fn create_magic_recovery_link(user_id: String) -> Result<String> {
    use crate::models::magic_link::{MagicLink, MagicLinkPurpose};
    use crate::models::user::UserDB;
    use chrono::Duration;
    use tracing::info;
    use uuid::Uuid;

    let admin_user_id = super::current_admin_user_id().await?;

    let user_id = Uuid::parse_str(&user_id).map_err(|_| crate::Error::Unauthorized)?;
    let user = UserDB::get(user_id).await?;

    if user.email.is_none() {
        return Err(crate::Error::NotFound);
    }

    let link_id = MagicLink::create_for_email(
        user_id,
        MagicLinkPurpose::Recovery,
        Duration::minutes(30),
        user.email.as_deref(),
    )
    .await
    .map_err(|_| crate::Error::InternalError)?;

    info!(
        admin_user_id = %admin_user_id,
        target_user_id = %user_id,
        link_id = %link_id,
        purpose = "recovery",
        "Admin created magic link."
    );

    Ok(format!("{}/l/{}", app_base_url(), link_id))
}

#[server]
pub async fn send_magic_invite_email(user_id: String) -> Result<String> {
    use crate::models::magic_link::{MagicLink, MagicLinkPurpose};
    use crate::models::user::UserDB;
    use crate::service::email::{send_magic_link, MagicLinkEmailKind};
    use chrono::Duration;
    use tracing::info;
    use uuid::Uuid;

    let admin_user_id = super::current_admin_user_id().await?;
    let user_id = Uuid::parse_str(&user_id).map_err(|_| crate::Error::Unauthorized)?;
    let user = UserDB::get(user_id).await?;
    let Some(email) = user.email else {
        return Err(crate::Error::NotFound);
    };

    let link_id = MagicLink::create_for_email(
        user_id,
        MagicLinkPurpose::Invite,
        Duration::days(14),
        Some(&email),
    )
    .await
    .map_err(|_| crate::Error::InternalError)?;
    let link = format!("{}/l/{}", app_base_url(), link_id);

    send_magic_link(&email, &link, MagicLinkEmailKind::Invite).await?;

    info!(
        admin_user_id = %admin_user_id,
        target_user_id = %user_id,
        link_id = %link_id,
        purpose = "invite",
        "Admin sent magic link email."
    );

    Ok("Invite email queued.".to_string())
}

#[server]
pub async fn send_magic_recovery_email(user_id: String) -> Result<String> {
    use crate::models::magic_link::{MagicLink, MagicLinkPurpose};
    use crate::models::user::UserDB;
    use crate::service::email::{send_magic_link, MagicLinkEmailKind};
    use chrono::Duration;
    use tracing::info;
    use uuid::Uuid;

    let admin_user_id = super::current_admin_user_id().await?;
    let user_id = Uuid::parse_str(&user_id).map_err(|_| crate::Error::Unauthorized)?;
    let user = UserDB::get(user_id).await?;
    let Some(email) = user.email else {
        return Err(crate::Error::NotFound);
    };
    if user.email_verified_at.is_none() {
        return Err(crate::Error::Unauthorized);
    }

    let link_id = MagicLink::create_for_email(
        user_id,
        MagicLinkPurpose::Recovery,
        Duration::minutes(30),
        Some(&email),
    )
    .await
    .map_err(|_| crate::Error::InternalError)?;
    let link = format!("{}/l/{}", app_base_url(), link_id);

    send_magic_link(&email, &link, MagicLinkEmailKind::Recovery).await?;

    info!(
        admin_user_id = %admin_user_id,
        target_user_id = %user_id,
        link_id = %link_id,
        purpose = "recovery",
        "Admin sent magic link email."
    );

    Ok("Recovery email queued.".to_string())
}

#[server]
pub async fn send_email_verification(user_id: String) -> Result<String> {
    use crate::models::magic_link::{MagicLink, MagicLinkPurpose};
    use crate::models::user::UserDB;
    use crate::service::email::{send_magic_link, MagicLinkEmailKind};
    use chrono::Duration;
    use tracing::info;
    use uuid::Uuid;

    let admin_user_id = super::current_admin_user_id().await?;
    let user_id = Uuid::parse_str(&user_id).map_err(|_| crate::Error::Unauthorized)?;
    let user = UserDB::get(user_id).await?;
    let Some(email) = user.email else {
        return Err(crate::Error::NotFound);
    };

    let link_id = MagicLink::create_for_email(
        user_id,
        MagicLinkPurpose::EmailVerification,
        Duration::hours(24),
        Some(&email),
    )
    .await
    .map_err(|_| crate::Error::InternalError)?;
    let link = format!("{}/email/verify/{}", app_base_url(), link_id);

    send_magic_link(&email, &link, MagicLinkEmailKind::Verification).await?;

    info!(
        admin_user_id = %admin_user_id,
        target_user_id = %user_id,
        link_id = %link_id,
        purpose = "email_verification",
        "Admin sent email verification link."
    );

    Ok("Verification email queued.".to_string())
}

#[server]
pub async fn verify_email(link: String) -> Result<String> {
    use crate::models::magic_link::MagicLink;
    use crate::models::user::mark_user_email_verified;
    use tracing::{error, info};

    let Ok(magic_link) = MagicLink::consume(&link).await else {
        error!("COULD NOT GET EMAIL VERIFICATION LINK");
        return Err(crate::Error::Unauthorized);
    };

    if magic_link.purpose != "email_verification" {
        return Err(crate::Error::Unauthorized);
    }

    let Some(email) = magic_link.email else {
        return Err(crate::Error::Unauthorized);
    };

    mark_user_email_verified(magic_link.user_id, &email).await?;

    info!(
        target_user_id = %magic_link.user_id,
        link_id = %magic_link.id,
        purpose = "email_verification",
        "Email verified."
    );

    Ok("Email verified.".to_string())
}

#[cfg(feature = "ssr")]
fn app_base_url() -> String {
    std::env::var("APP_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string())
        .trim_end_matches('/')
        .to_string()
}
