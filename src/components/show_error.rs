use leptos::prelude::*;

#[component]
pub fn ShowError(error: Memo<Option<ServerFnError>>) -> impl IntoView {
    view! {
        <Show when=move || { error.get().is_some() }>
            <div data-state="error">
                {move || error.get().as_ref().map(|e| view! { <p>{e.to_string()}</p> })}
            </div>
        </Show>
    }
}
