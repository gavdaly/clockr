use leptos::prelude::*;

#[server]
pub async fn magic_sign_in(link: String) -> Result<(), ServerFnError> {
    use crate::models::magic_link::MagicLink;
    use leptos::prelude::server_fn::error::*;
    use tracing::{error, info};

    let (_, session) = super::current_user()
        .await
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    info!("Session: {:?}", session);

    let Ok(user_id) = MagicLink::get(&link).await else {
        error!("COULD NOT GET USER FROM MAGIC LINK");
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "The link has expired".into(),
        ));
    };

    info!("Signed in user id: {} with magic link.", user_id);

    session.set_longterm(true);
    session.set("id", user_id);
    leptos_axum::redirect("/app");

    Ok(())
}
