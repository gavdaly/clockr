use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use crate::functions::MagicSignIn;

#[derive(Clone, Params, PartialEq)]
struct MagicLinkParams {
    link: Option<String>,
}

#[component]
pub fn MagicLink() -> impl IntoView {
    let params = use_params::<MagicLinkParams>();

    match params.get() {
        Ok(MagicLinkParams { link: Some(link) }) => view! { <MagicClick link/> }.into_any(),
        Err(e) => view! { <div>"Error parsing Parameters: " {e.to_string()}</div> }.into_any(),
        _ => view! { <div>"The link has expired, please try again!"</div> }.into_any(),
    }
}

#[component]
fn MagicClick(link: String) -> impl IntoView {
    let magic_sign_in = ServerAction::<MagicSignIn>::new();
    view! {
        <ActionForm action=magic_sign_in>
            <input type="hidden" name="link" value=link/>
            <button type="submit">Sign In</button>
        </ActionForm>
    }
}
