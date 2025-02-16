use chrono::Timelike;
use leptos::prelude::*;
use leptos::{form::ActionForm, tachys::dom::window};

use crate::{functions::user::CheckIn, models::user::UserDisplay};

#[derive(Clone, Debug)]
struct TimeEntry {
    id: String,
    time: String,
    timestamp: u64,
}

/// Renders the home page of your application.
#[component]
pub fn Dashboard() -> impl IntoView {
    let (user, set_user) = signal::<Option<UserDisplay>>(None);

    let timestamp = |time| {
        chrono::NaiveTime::parse_from_str(time, "%H:%M")
            .unwrap()
            .num_seconds_from_midnight() as u64
    };

    let (times, set_times) = signal(vec![
        TimeEntry {
            id: "1".to_string(),
            time: "8:03 AM".to_string(),
            timestamp: timestamp("08:03"),
        },
        TimeEntry {
            id: "2".to_string(),
            time: "12:00 PM".to_string(),
            timestamp: timestamp("12:00"),
        },
        TimeEntry {
            id: "3".to_string(),
            time: "3:00 PM".to_string(),
            timestamp: timestamp("15:00"),
        },
    ]);

    let delete_item = move |id: String| {
        set_times.update(|t| {
            t.retain(|entry| entry.id != id);
        });
    };

    let today = Memo::new(move |_| {
        let t = times.get();
        let len = t.len();
        if len == 0 {
            return 0;
        }
        let prev: u64 = t
            .chunks(2)
            .map(|te| {
                if te.len() == 2 {
                    te[1].timestamp - te[0].timestamp
                } else {
                    0
                }
            })
            .sum();

        if len % 2 != 0 {
            let _last_entry = t.last().unwrap();
        }
        prev
    });
    let week = Memo::new(move |_| 23554);
    view! {
        <section class="stack">
            <ClkIn />
            <div id="check-ins" class="card wide">
                <h2>"Check Ins"</h2>
                <p> "add time +"</p>
                <ul class="slide-list">
                            {move || times.get().into_iter().map(|entry| {

                                view! {
                                    <li>
                                        <time datetime="">{entry.time}</time>
                                        <div class="delete-indicator">Delete</div>
                                    </li>
                                }
                            }).collect::<Vec<_>>()}
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
