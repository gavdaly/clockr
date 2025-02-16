use leptos::form::ActionForm;
use leptos::prelude::*;

use crate::{functions::user::CheckIn, models::user::UserDisplay};

/// Renders the home page of your application.
#[component]
pub fn Dashboard() -> impl IntoView {
    let (user, set_user) = signal::<Option<UserDisplay>>(None);
    view! {
        <section class="stack">
            <ClkIn />
            <div id="check-ins">
                <h2>"Check Ins"</h2>
                <ul>
                    <li><time>"8:00 AM"</time></li>
                    <li><time>"12:00 PM"</time></li>
                </ul>
            </div>
            <div id="today-hours">
                <h2>"Today Hours"</h2>
                <time id="today-hours-display">"00:00"</time>
            </div>
            <div id="total-hours">
                <h2>"Total Hours"</h2>
                <time id="total-hours-display">"00:00"</time>
            </div>
        </section>
    }
}

#[island]
fn ClkIn() -> impl IntoView {
    let action = ServerAction::<CheckIn>::new();
    view! {
        <ActionForm action>
            <button id="checked_in" data-checked-in="false">
                {if false { "You are Checked In" } else { "You are Checked Out" }}
            </button>
        </ActionForm>
    }
}
