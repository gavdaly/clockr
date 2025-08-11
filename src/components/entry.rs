use crate::components::adjustment::Adjustment;
use crate::components::session::Session;
use crate::models::time_sheets::Entry;
use leptos::prelude::*;

/// Renders a view based on the type of the model `time_sheets::Entry`. Which displays a `Session` or `Adjustment`.
#[component]
pub fn Entry(entry: Entry) -> impl IntoView {
    match entry {
        Entry::Session(session) => view! { <Session session/> }.into_any(),
        Entry::Adjustment(adjustment) => view! { <Adjustment adjustment/> }.into_any(),
    }
}
