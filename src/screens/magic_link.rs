use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

#[derive(Clone, Params, PartialEq)]
struct MagicLinkParams {
    link: Option<String>,
}

#[component]
pub fn MagicLink() -> impl IntoView {
    let params = use_params::<MagicLinkParams>();

    match params.get() {
        Ok(MagicLinkParams { link: Some(link) }) => view! {
            <MagicClick link />
        }
        .into_any(),
        Err(e) => view! { <div>"Error parsing Parameters: " {e.to_string()}</div> }.into_any(),
        _ => view! {<div>"The link has expired, please try again!"</div>}.into_any(),
    }
}

#[island]
fn MagicClick(link: String) -> impl IntoView {
    let magic_sign_in = ServerAction::<MagicSignIn>::new();
    view! {
        <ActionForm action=magic_sign_in>
            <input type="hidden" name="link" value={link} />
            <button type="submit">Sign In</button>
        </ActionForm>
    }
}

#[server]
async fn magic_sign_in(link: String) -> Result<(), ServerFnError> {
    use crate::models::magic_link::MagicLink;
    use axum_session::SessionAnySession;
    use leptos::prelude::server_fn::error::*;
    use tracing::{error, info};

    let Some(session) = use_context::<SessionAnySession>() else {
        error!("COULD NOT GET SESSION CONTEXT");
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "Session missing.".into(),
        ));
    };

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
