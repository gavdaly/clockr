use leptos::prelude::*;
use leptos_router::components::A;

// use crate::models::user::UserDisplay;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // get settings
    // show week summary
    // upcomming vacations
    view! {
        <section class="stack">
            <A href="/app/check_in">
                {move || {
                    view! {
                        <aside id="checked_in" data-checked-in="true">
                            {if true { "You are Checked In" } else { "You are Checked Out" }}
                        </aside>
                    }
                }}

            </A>
        </section>
    }
}
