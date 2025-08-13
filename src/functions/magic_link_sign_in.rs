use crate::Result;
use leptos::prelude::*;

#[server]
pub async fn magic_sign_in(link: String) -> Result<()> {
    use crate::models::magic_link::MagicLink;
    use tracing::{error, info};

    let (_, session) = super::current_user()
        .await
        .ok_or(crate::Error::Unauthorized)?;

    info!("Session: {:?}", session);

    let Ok(user_id) = MagicLink::get(&link).await else {
        error!("COULD NOT GET USER FROM MAGIC LINK");
        return Err(crate::Error::Unauthorized);
    };

    info!("Signed in user id: {} with magic link.", user_id);

    session.set_longterm(true);
    session.set("id", user_id);
    leptos_axum::redirect("/app");

    Ok(())
}
