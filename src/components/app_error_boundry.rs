use leptos::prelude::*;

#[component]
pub fn AppErrorBoundary(
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();

    view! {
        <ErrorBoundary
            fallback=move |errors| {
                view! {
                    <div class=format!("error-boundary {class}") role="alert">
                        <h2>"Something went wrong."</h2>
                        <ul>
                            {move || errors
                                .get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    </div>
                }
            }
        >
            {children()}
        </ErrorBoundary>
    }
}
