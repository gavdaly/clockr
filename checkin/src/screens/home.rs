use leptos::*;
use leptos_router::A;

/// Renders the home page of your application.
#[component]
pub fn HomePage<F>(status: F) -> impl IntoView where F: Fn() -> bool + 'static {

    // get settings
    // show week summary
    // upcomming vacations
    view! {
        <section class="stack">
            <A href="/check_in">
                {move || {
                    view! {
                        <aside id="checked_in" data-checked-in=status().to_string()>
                            {if status() { "In" } else { "Out" }}
                        </aside>
                    }
                }}
            </A>
        </section>
    }
}

#[component]
pub fn Settings() -> impl IntoView {
    view! {
        <section class="stack">
            <h1>"Settings | To Do"</h1>
        </section>
    }
}
