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
pub fn ClockInLink() -> impl IntoView {
    let params = use_params::<ClockInLinkParams>();
    // let clock_in_link = ServerAction::<ClockInLinkInitiateSession>::new();
    match params.get() {
        Ok(ClockInLinkParams { link: Some(_link) }) => {
            // clock_in_link.dispatch(ClockInLinkInitiateSession { link });
            view! {
                <div>
                    <Loading/>
                    <Redirect path="/app"/>
                </div>
            }
            .into_any()
        }
        Err(e) => view! { <div>"Something went wrong: " {e.to_string()}</div> }.into_any(),
        _ => view! { <div>"Could not find Link"</div> }.into_any(),
    }
}
