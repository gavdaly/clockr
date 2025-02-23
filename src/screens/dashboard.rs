use leptos::prelude::*;
use crate::components::ShowError;
use crate::models::TimeLog;

/// Renders the home page of your application.
#[component]
pub fn Dashboard() -> impl IntoView {
    let (times, _set_times) = signal(vec![
        TimeLog::new("1".to_string(), 8 * 3600 + 3 * 60),      // 08:03 = (8 * 3600) + (3 * 60) = 29,580 seconds
        TimeLog::new("1".to_string(), 12 * 3600),              // 12:00 = (12 * 3600) = 43,200 seconds
        TimeLog::new("1".to_string(), 15 * 3600),              // 15:00 = (15 * 3600) = 54,000 seconds
    ]);

    let today = Memo::new(move |_| {
        let t = times.get();
        let len = t.len();
        if len == 0 {
            return 0;
        }
        let prev = t
            .chunks(2)
            .map(|te| {
                if te.len() == 2 {
                    te[1].event_time - te[0].event_time
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
            <ClkIn/>
            <div id="check-ins" class="card wide">
                <div>
                    <h2>"Check Ins"</h2>
                    <a href="/app/time/add">"add time +"</a>
                </div>
                <ul class="slide-list">
                    {move || {
                        times
                            .get()
                            .into_iter()
                            .map(|entry| {
                                view! {
                                    <li data-id=entry.id>
                                        <time datetime="">{entry.event_time}</time>
                                        <div class="delete-indicator">Delete</div>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}

                </ul>
            </div>
            <div class="card small">
                <h2>"Today"</h2>
                <DurationHourDisplay seconds=today/>
            </div>
            <div class="card small">
                <h2>"Week"</h2>
                <DurationHourDisplay seconds=week/>
            </div>
        </section>
    }
}

#[component]
fn DurationHourDisplay(seconds: Memo<i64>) -> impl IntoView {
    let h = seconds.get() / 3600;
    let m = (seconds.get() % 3600) / 60;
    let s = seconds.get() % 60;
    let hours = seconds.get() as f32 / 3600.0;
    let time = format!("PT{h}H{m}M{s}S");
    view! { <time datetime=time>{format!("{:.2}h", hours)}</time> }
}

#[island]
fn ClkIn() -> impl IntoView {
    use crate::functions::user::CheckIn;
    use leptos::{form::ActionForm, tachys::dom::window};

    let action = ServerAction::<CheckIn>::new();
    let result = action.value();
    let error = Memo::new(move |_| result.get().and_then(|r| r.err()));
    let (disabled, set_disabled) = signal(false);

    Effect::new(move |_| {
        if let Some(Ok(_)) = result.get() {
            let _ = window().navigator().vibrate_with_duration(100);
            set_disabled.set(true);
            set_timeout(
                move || {
                    set_disabled.set(false);
                },
                std::time::Duration::from_secs(10),
            );
        }
    });

    view! {
        <ActionForm action attr:id="clk-in">
            <button id="checked_in" data-checked-in="true" prop:disabled=disabled>
                {if true { "You are Checked In" } else { "You are Checked Out" }}
            </button>
        </ActionForm>
        <ShowError error=error/>
    }
}


