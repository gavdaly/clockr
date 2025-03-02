use super::Icon;
use leptos::prelude::*;

/// Renders a loading state with an icon and a text.
///
/// # Returns
/// A view representing a loading state with an icon and a text.
#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div data-state="loading">
            <Icon name="loading"/>
            <span>"Loading..."</span>
        </div>
    }
}
