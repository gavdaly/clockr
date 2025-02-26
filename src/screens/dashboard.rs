use leptos::prelude::*;
use crate::functions::user::get_current_user;
use crate::models::user::CurrentUser;
use crate::components::{ShowError, Loading};
use crate::models::TimeLog;


/// Renders the home page of your application.
#[component]
pub fn Dashboard() -> impl IntoView {
    let (show_time, set_show_time) = signal(false);

    let user_resource = Resource::new(
        || (),
        async move |_| {
            match get_current_user().await {
                Ok(u) => u,
                Err(e) => {
                    tracing::error!("Failed to get current user: {}", e);
                    CurrentUser::Guest
                }
            }
        }
    );

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
                    <button on:click=move |_| {
                        set_show_time.set(!show_time.get())
                    }>"add time +"</button>
                </div>
                <Show when=move || show_time.get()>

                    <div>
                        <form>
                            <div>
                                <label for="time">"Time"</label>
                                <input type="time" name="time" id="time"/>
                            </div>
                            <div>
                                <label for="reason">"Reason"</label>
                                <textarea name="reason" id="reason"></textarea>
                            </div>
                            <button type="submit">"Submit"</button>
                        </form>
                    </div>
                </Show>

                <ul class="slide-list">
                    {move || {
                        times
                            .get()
                            .into_iter()
                            .map(|entry| {
                                view! { <DurationDisplay entry=entry/> }
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
fn DurationDisplay(entry: TimeLog) -> impl IntoView {
    use leptos::ev::MouseEvent;
    let delete_entry = |e: MouseEvent| {
        e.prevent_default();
    };
    view! {
        <li data-entry-id=entry.id>
            <time datetime="">{entry.event_time}</time>
            <span on:click=delete_entry class="delete-indicator">
                Delete
            </span>
        </li>
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

#[component]
fn ClkIn() -> impl IntoView {
    use crate::functions::CheckIn;
    use crate::models::CurrentUser;
    use leptos::{form::ActionForm, tachys::dom::window};

     let user= use_context::<CurrentUser>();
    let checked_in = Signal::derive(move || match user.clone() {
        Some(CurrentUser::Authenticated(user)) => user.check_ins.len() % 2 == 1,
        _ => {
            false
        }

    }
    );
    let checked_in_text = Signal::derive(move || checked_in.get().to_string());

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
            <Suspense fallback=move || view! { <Loading/> }>
                <button
                    id="checked_in"
                    data-checked-in=move || checked_in_text.get()
                    prop:disabled=disabled
                >
                    {move || {
                        if checked_in.get() { "You are Checked In" } else { "You are Checked Out" }
                    }}

                </button>
            </Suspense>
        </ActionForm>
        <ShowError error/>
    }
}


