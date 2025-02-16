use leptos::form::ActionForm;
use leptos::prelude::*;

use crate::{functions::user::CheckIn, models::user::UserDisplay};

/// Renders the home page of your application.
#[component]
pub fn Dashboard() -> impl IntoView {
    let (user, set_user) = signal::<Option<UserDisplay>>(None);
    let today = Memo::new(move |_| 7326);
    let week = Memo::new(move |_| 23554);
    view! {
        <section class="stack">
            <ClkIn />
            <div id="check-ins" class="card wide">
                <h2>"Check Ins"</h2>
                <p> add time </p>
                <ul>
                    <li><time datetime="">"8:00 AM"</time></li>
                    <li><time datetime="">"12:00 PM"</time></li>
                </ul>
            </div>
            <div class="card small">
                <h2>"Today"</h2>
                <DurationHourDisplay seconds={today} />
            </div>
            <div class="card small">
                <h2>"Week"</h2>
                <DurationHourDisplay seconds={week} />
            </div>
        </section>
    }
}

#[component]
fn DurationHourDisplay(seconds: Memo<u64>) -> impl IntoView {
    let h = seconds.get() / 3600;
    let m = (seconds.get() % 3600) / 60;
    let s = seconds.get() % 60;
    let hours = seconds.get() as f32 / 3600.0;
    let time = format!("PT{h}H{m}M{s}S");
    view! {<time datetime={time}>{format!("{:.2}h", hours)}</time>}
}

#[island]
fn ClkIn() -> impl IntoView {
    let action = ServerAction::<CheckIn>::new();
    let result = action.value();
    let error = Memo::new(move |_| result.get().and_then(|r| r.err()));

    if let Some(Ok(_)) = result.get_untracked() {
        // also play a sound
        let _ = window().navigator().vibrate_with_duration(100);
    }

    view! {
        <ActionForm action attr:id="clk-in">
            <button id="checked_in" data-checked-in="true">
                {if true { "You are Checked In" } else { "You are Checked Out" }}
            </button>
        </ActionForm>
        <ShowError error={error} />
    }
}

#[component]
fn ShowError(error: Memo<Option<ServerFnError>>) -> impl IntoView {
    view! {
        <div class="error">
            {move || error.get().as_ref().map(|e| view! {<p>{e.to_string()}</p>})}
        </div>

    }
}
