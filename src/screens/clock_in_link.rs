use crate::components::loading_progress::Loading;
use leptos::prelude::*;
use leptos_router::components::Redirect;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

#[derive(Clone, Params, PartialEq)]
struct ClockInLinkParams {
    link: Option<String>,
}

#[component]
pub fn ClockInLink(clock_in_link: ServerAction<ClockInLinkInitiateSession>) -> impl IntoView {
    let params = use_params::<ClockInLinkParams>();
    match params.get() {
        Ok(ClockInLinkParams { link: Some(link) }) => {
            clock_in_link.dispatch(ClockInLinkInitiateSession { link });
            view! {
                <div>
                    <Loading/>
                    <Redirect path="/app"/>
                </div>
            }
            .into_any()
        }
        Err(e) => view! { <div>"Something went wrong: " {e.to_string()}</div> }.into_any(),
        _ => view! {<div>"Could not find Link"</div>}.into_any(),
    }
}

#[server]
pub async fn clock_in_link_initiate_session(link: String) -> Result<(), ServerFnError> {
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

    // check to see if link is valid!!
    leptos::logging::log!("link: {link}");

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
