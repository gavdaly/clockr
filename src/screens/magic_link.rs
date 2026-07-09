use crate::functions::{MagicSignIn, VerifyEmail};
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

#[derive(Clone, Params, PartialEq)]
struct MagicLinkParams {
    link: Option<String>,
}

#[derive(Clone, Params, PartialEq)]
struct EmailVerificationParams {
    link: Option<String>,
}

#[component]
pub fn EmailVerification() -> impl IntoView {
    let params = use_params::<EmailVerificationParams>();

    match params.get() {
        Ok(EmailVerificationParams { link: Some(link) }) => {
            view! { <EmailVerificationClick link/> }.into_any()
        }
        Err(e) => view! { <div>"Error parsing Parameters: " {e.to_string()}</div> }.into_any(),
        _ => view! { <div>"The verification link has expired, please try again!"</div> }.into_any(),
    }
}

#[component]
fn EmailVerificationClick(link: String) -> impl IntoView {
    let verify_email = ServerAction::<VerifyEmail>::new();
    let value = verify_email.value();

    view! {
        <ActionForm action=verify_email>
            <input type="hidden" name="link" value=link/>
            <button type="submit">"Verify Email"</button>
        </ActionForm>
        {move || {
            value
                .get()
                .map(|result| match result {
                    Ok(message) => view! { <p>{message}</p> }.into_any(),
                    Err(error) => view! { <p data-state="error">{error.to_string()}</p> }.into_any(),
                })
        }}
    }
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
