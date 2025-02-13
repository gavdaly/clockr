use crate::components::loading_progress::Loading;
use leptos::prelude::*;
use leptos_router::components::Redirect;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use uuid::Uuid;

#[derive(Clone, Params, PartialEq)]
struct MagicLinkParams {
    link: Option<String>,
}

#[component]
pub fn MagicLink() -> impl IntoView {
    let params = use_params::<MagicLinkParams>();
    let magic_sign_in = ServerAction::<MagicSignIn>::new();
    match params.get() {
        Ok(MagicLinkParams { link: Some(link) }) => {
            magic_sign_in.dispatch(MagicSignIn { link });
            view! {
                <div>
                    <Loading/>
                    <Redirect path="/app"/>
                </div>
            }
            .into_any()
        }
        Err(e) => view! { <div>"Error parsing Parameters: " {e.to_string()}</div> }.into_any(),
        _ => view! {<div>"The link has expired, please try again!"</div>}.into_any(),
    }
}

#[server]
async fn magic_sign_in(link: String) -> Result<(), ServerFnError> {
    use crate::models::magic_link::MagicLink;
    use axum_session::SessionAnySession;
    use leptos::prelude::server_fn::error::*;

    let session = use_context::<SessionAnySession>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Session missing.".into()))?;
    let user_id = MagicLink::get(link)
        .await
        .map_err(|_| ServerFnError::<NoCustomError>::ServerError("Invalid Link".into()))?;

    // find session
    leptos::logging::log!("magic_link: {link}");

    session.set_longterm(true);
    session.set("id", user_id);
    leptos_axum::redirect("/app");

    Ok(())
}
