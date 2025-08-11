use leptos::prelude::*;

/// Creates an SVG icon element with the specified class and href attributes.
///
/// # Arguments
///
/// * `name` - The name of the icon.
#[component]
pub fn Icon(#[prop(into)] name: String) -> impl IntoView {
    view! {
        <svg class=format!("icon {name}")>
            <use_ href=format!("/icons.svg#{name}")></use_>
        </svg>
    }
}
