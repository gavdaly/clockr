use leptos::prelude::*;

#[component]
pub fn AppErrorBoundary(children: Children) -> impl IntoView {
    view! {
        <ErrorBoundary
            fallback=move |errors| {
                view! {
                    <div class="error-boundary" role="alert">
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
