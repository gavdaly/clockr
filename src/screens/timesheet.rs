use crate::models::time_sheets::TimeSheet;
use leptos::prelude::*;

#[component]
pub fn TimeSheetDisplay() -> impl IntoView {
    use crate::functions::AddTime;
    let action = ServerAction::<AddTime>::new();

    let timesheet = Resource::new(move || action.version(), move |_| get_active_user_timesheet());
    
    let add_time_pending = action.pending();
    let selected_date = RwSignal::new(None::<String>);
    
    let show_add_form = RwSignal::new(false);
    
    
    let handle_add_time = move |date: String| {
        selected_date.set(Some(date));
        show_add_form.set(true);
    };
    
    let close_form = move |_| {
        show_add_form.set(false);
    };
    
    view! {
            <section class="stack">
                <h2>"Timesheet"</h2>
                
                {move || match timesheet.get() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(Err(e)) => view! { <p class="error">"Error loading timesheet: " {e.to_string()}</p> }.into_any(),
                    Some(Ok(ts)) => view! {
                        <div class="timesheet">
                            {ts.weeks.into_iter().map(|week| {
                                view! {
                                    <div class="week">
                                        <div class="week-header">
                                            <h3>"Week of " {week.start_date} " to " {week.end_date}</h3>
                                            <div class="week-total">"Total: " {format!("{:.2}", week.total_hours)} " hours"</div>
                                        </div>
                                        <div class="days">
                                            {week.days.into_iter().map(|day| {
                                                let day_date = day.date.clone();
                                                view! {
                                                    <div class="day">
                                                        <div class="day-header">
                                                            <h4>{day.date}</h4>
                                                            <div class="day-total">"Total: " {format!("{:.2}", day.total_hours)} " hours"</div>
                                                        </div>
                                                        <div class="entries">
                                                            {day.entries.is_empty().then(|| view! {
                                                                <p class="no-entries">"No time entries for this day"</p>
                                                            })}
                                                            {day.entries.into_iter().map(|entry| {
                                                                let start_time = chrono::DateTime::from_timestamp(entry.start_time, 0)
                                                                    .map(|dt| dt.format("%H:%M").to_string())
                                                                    .unwrap_or_else(|| "Invalid time".to_string());
                                                                
                                                                let end_time = entry.end_time
                                                                    .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
                                                                    .map(|dt| dt.format("%H:%M").to_string())
                                                                    .unwrap_or_else(|| "--:--".to_string());
                                                                
                                                                let duration = entry.duration
                                                                    .map(|d| format!("{:.2} hours", d as f64 / 3600.0))
                                                                    .unwrap_or_else(|| "In progress".to_string());
                                                                
                                                                let correction_status = entry.correction.as_ref().map(|c| {
                                                                    match c.state {
                                                                        0 => "(Pending)",
                                                                        1 => "(Approved)",
                                                                        2 => "(Rejected)",
                                                                        10 => "(Admin Added)",
                                                                        -1 => "(Deleted by User)",
                                                                        -10 => "(Deleted by Admin)",
                                                                        _ => ""
                                                                    }
                                                                }).unwrap_or("");
                                                                
                                                                view! {
                                                                    <div class="entry">
                                                                        <div class="entry-times">
                                                                            <span class="start-time">{start_time}</span>
                                                                            <span class="time-separator">" - "</span>
                                                                            <span class="end-time">{end_time}</span>
                                                                        </div>
                                                                        <div class="entry-duration">{duration}</div>
                                                                        <div class="entry-status">{correction_status}</div>
                                                                        {entry.correction.as_ref().map(|c| {
                                                                            view! {
                                                                                <div class="entry-reason">{c.reason.clone()}</div>
                                                                            }
                                                                        })}.into_any()
                                                                    </div>
                                                                }
                                                            }).collect::<Vec<_>>()}
                                                        </div>
                                                        <button 
                                                            class="add-entry-btn"
                                                            on:click=move |_| handle_add_time(day_date.clone())
                                                        >
                                                            "+ Add Time"
                                                        </button>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }}
                
                {move || show_add_form.get().then(|| {
                    view! {
                        <div class="add-time-form-overlay">
                            <div class="add-time-form">
                                <h3>"Add Time for " {selected_date.get().unwrap_or_default()}</h3>
                                <ActionForm action>
                                    <input type="hidden" id="date" value=selected_date.get().unwrap_or_default() />
                                    <div class="form-group">
                                        <label for="time">"Time (HH:MM)"</label>
                                        <input 
                                            type="time" 
                                            id="time" 
                                            required
                                        />
                                    </div>
                                    <div class="form-group">
                                        <label for="reason">"Reason"</label>
                                        <textarea 
                                            id="reason" 
                                            required
                                        ></textarea>
                                    </div>
                                    <div class="form-actions">
                                        <button type="reset" on:click=close_form>"Cancel"</button>
                                        <button 
                                            type="submit" 
                                            prop:disabled=add_time_pending.get()
                                        >
                                            {move || if add_time_pending.get() { "Adding..." } else { "Add Time" }}
                                        </button>
                                    </div>
                                </ActionForm>
                            </div>
                        </div>
                    }
                })}
            </section>
    }
}



#[server]
async fn get_active_user_timesheet() -> Result<TimeSheet, ServerFnError> {
    use axum_session::SessionAnySession;
    use chrono::{Duration, Local, Weekday};
    use uuid::Uuid;

    let Some(session) = use_context::<SessionAnySession>() else {
        return Err(ServerFnError::ServerError("Session missing.".into()));
    };
    let Some(id) = session.get::<Uuid>("id") else {
        leptos_axum::redirect("/sign_in");
        return Err(ServerFnError::ServerError("Error getting Session!".into()));
    };

    let now = Local::now().naive_local();
    let three_weeks_before = now.clone().date().week(Weekday::Mon).first_day() - Duration::days(14);
    let end_of_week = now.date().week(Weekday::Mon).last_day() + Duration::days(7);

    match TimeSheet::generate_for(id, three_weeks_before, end_of_week).await {
        Ok(ts) => {
            tracing::info!("{:?}", ts);
            Ok(ts)
        }
        Err(_) => Err(ServerFnError::ServerError(
            "Error Generating Time Sheet".into(),
        )),
    }
}
