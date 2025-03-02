use crate::components::{Loading, ShowError};
use crate::models::{CurrentUser, TimeLog, UserToday};
use leptos::prelude::*;

/// Renders the home page of your application.
#[component]
pub fn Dashboard() -> impl IntoView {
    let (show_time, set_show_time) = signal(false);

    let user_resource =
        use_context::<Resource<CurrentUser>>().expect("Not wrapped in `UserProvider`");

    let week = Signal::derive(move || 23554);
    view! {
        <section class="stack">
            <Suspense fallback=move || {
                view! { <Loading/> }
            }>
                {move || match user_resource.read().clone() {
                    Some(CurrentUser::Authenticated(user)) => {
                        let u = Signal::derive(move || user.clone());
                        view! { <ClkIn user=u/> }.into_any()
                    }
                    _ => view! {}.into_any(),
                }}

            </Suspense>
            <div id="check-ins" class="card wide">
                <div>
                    <h2>"Check Ins"</h2>
                    <button on:click=move |_| {
                        set_show_time.set(!show_time.get())
                    }>"add time +"</button>
                </div>
                <Show when=move || show_time.get()>
                    <AddTimeForm on_success=Callback::new(move |_| set_show_time.set(false)) />
                </Show>
                <ul class="slide-list">
                    <Suspense fallback=move || {
                        view! { <Loading/> }
                    }>
                        {move || match user_resource.read().clone() {
                            Some(CurrentUser::Authenticated(user)) => {
                                let times = user.check_ins.clone();
                                times
                                    .iter()
                                    .map(|entry| {
                                        view! { <LogEntry entry=entry.clone()/> }
                                    })
                                    .collect::<Vec<_>>()
                                    .into_any()
                            }
                            _ => view! {}.into_any(),
                        }}
                    </Suspense>
                </ul>
            </div>

            <Suspense fallback=move || {
                view! { <Loading/> }
            }>
                {move || match user_resource.read().clone() {
                    Some(CurrentUser::Authenticated(user)) => {
                        let logs = Signal::derive(move || user.check_ins.clone());
                        view! { <TodayTotal logs=logs/> }.into_any()
                    }
                    _ => view! {}.into_any(),
                }}

            </Suspense>

            <div class="card small">
                <h2>"Week"</h2>
                <DurationDateTime seconds=week/>
            </div>
        </section>
    }
}

#[component]
fn AddTimeForm(
    #[prop(optional, into, default = None)]
    date: Option<String>,
    on_success: Callback<()>) -> impl IntoView {
    use crate::functions::AddTime;
    use leptos::form::ActionForm;
    use chrono::Local;

    let date = match date {
        Some(date) => date,
        None => Local::now().date_naive().to_string()
    };
    
    let action = ServerAction::<AddTime>::new();
    let pending = action.pending();
    let result = action.value();
    
    let error = Memo::new(move |_| result.get().and_then(|r| r.err()));
    
    // Call on_success when the action completes successfully
    Effect::new(move |_| {
        if let Some(Ok(_)) = result.get() {
            on_success.run(());
        }
    });
    
    view! {
        <div>
            <ActionForm action>
                <input type="hidden" name="date" value=date />
                <div>
                    <label for="time">"Time"</label>
                    <input 
                        type="time" 
                        name="time" 
                        id="time"
                        required
                    />
                </div>
                <div>
                    <label for="reason">"Reason"</label>
                    <textarea 
                        name="reason" 
                        id="reason"
                        required
                    ></textarea>
                </div>
                <button 
                    type="submit"
                    prop:disabled=pending
                >
                    {move || if pending.get() { "Submitting..." } else { "Submit" }}
                </button>
            </ActionForm>
            <ShowError error/>
        </div>
    }
}

/// Component that calculates and displays the total time logged for today
///
/// This component:
/// 1. Calculates completed time intervals (pairs of check-ins)
/// 2. Tracks ongoing time if user is currently checked in
/// 3. Updates the display every 10 seconds
///
/// # Arguments
///
/// * `logs` - A signal containing the user's time log entries
#[component]
pub fn TodayTotal(logs: Signal<Vec<TimeLog>>) -> impl IntoView {
    use web_sys::js_sys::Date;
    let (current_time, set_current_time) = signal(Date::now() as i64);

    let today = Signal::derive(move || {
        logs.get()
            .chunks(2)
            .map(|te| {
                if te.len() == 2 {
                    te[1].event_time - te[0].event_time
                } else {
                    0
                }
            })
            .sum::<i64>()
    });

    let remaining = Signal::derive(move || match logs.get().len() % 2 {
        1 => {
            logs.get()
                .last()
                .expect("should have last entry")
                .event_time
        }
        _ => 0,
    });

    set_interval(
        move || {
            let now = Date::now() as i64;
            set_current_time.set(now);
        },
        core::time::Duration::from_secs(10),
    );

    let total_seconds = Signal::derive(move || {
        let base = today.get();
        let mut elapsed = 0;
        if remaining.get() > 0 {
            elapsed = current_time.get() - remaining.get();
        }
        base + elapsed
    });

    view! {
        <div class="card small">
            <h2>"Today"</h2>
            <DurationDateTime seconds=total_seconds/>
        </div>
    }
}

#[component]
fn LogEntry(entry: TimeLog) -> impl IntoView {
    use crate::functions::DeleteTime;
    use leptos::form::ActionForm;

    let entry = Signal::derive(move || entry.clone());
    
    let action = ServerAction::<DeleteTime>::new();
    let pending = action.pending();
    let error = Memo::new(move |_| action.value().get().and_then(|r| r.err()));
    
    view! {
        <li data-entry-id={entry.get().id}>
            <time datetime="">{entry.get().event_time}</time>
            <ActionForm action>
                <input type="hidden" name="time_log_id" value=entry.get().id />
                <button 
                    type="submit"
                    class="delete-indicator"
                    prop:disabled=pending
                >
                    {move || if pending.get() { "Deleting..." } else { "Delete" }}
                </button>
            </ActionForm>
            <ShowError error/>
        </li>
    }
}

#[component]
fn DurationDateTime(seconds: Signal<i64>) -> impl IntoView {
    let h = seconds.get() / 3600;
    let m = (seconds.get() % 3600) / 60;
    let s = seconds.get() % 60;
    let hours = seconds.get() as f32 / 3600.0;
    let time = format!("PT{h}H{m}M{s}S");
    view! { <time datetime=time>{format!("{:.2}h", hours)}</time> }
}

#[component]
fn ClkIn(user: Signal<UserToday>) -> impl IntoView {
    use crate::functions::CheckIn;
    use leptos::{form::ActionForm, tachys::dom::window};

    let checked_in = Signal::derive(move || user.get().check_ins.len() % 2 == 1);
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
                core::time::Duration::from_secs(10),
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
