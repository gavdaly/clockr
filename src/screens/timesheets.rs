use crate::components::loading_progress::Loading;
// use crate::components::timesheet::TimeSheetDisplay;
use crate::models::time_sheets::TimeSheet;
use crate::models::user::UserToday;
use chrono::NaiveDate;
use leptos::prelude::*;
use leptos_router::components::A;
use uuid::Uuid;

/// Renders the home page of your application.
#[component]
pub fn TimeSheetsLayout(children: Children) -> impl IntoView {
    view! {
        <nav class="subWrapper">
            <a href="/app/timesheets">"Time Sheets"</a>
            <a href="/app/timesheets/adjustment">"Add Adjustment"</a>
            <a href="/app/timesheets/pending">"Pending Corrections"</a>
        </nav>
        <section class="stack">{children()}</section>
    }
}

#[server]
async fn load_timesheet_for<'a>(user_id: String) -> Result<TimeSheet, ServerFnError> {
    use chrono::{Duration, Local, Weekday};
    use uuid::Uuid;

    let Ok(id) = Uuid::parse_str(&user_id) else {
        return Err(ServerFnError::Deserialization("Error parsing ID".into()));
    };

    let now = Local::now().naive_local();
    let three_weeks_before = now.clone().date().week(Weekday::Mon).first_day() - Duration::days(14);
    let end_of_week = now.date().week(Weekday::Mon).last_day() + Duration::days(7);

    match TimeSheet::generate_for(id, three_weeks_before, end_of_week).await {
        Ok(ts) => Ok(ts),
        Err(_) => Err(ServerFnError::ServerError(
            "Error Generating Time Sheet".into(),
        )),
    }
}

#[server]
pub async fn load_hourly_users() -> Result<Vec<UserToday>, ServerFnError> {
    match UserToday::get_all_hourly().await {
        Ok(v) => Ok(v),
        Err(_) => Err(ServerFnError::ServerError("Server Error".to_string())),
    }
}

#[component]
pub fn TimeSheetsList() -> impl IntoView {
    let (current_user, set_current_user) = signal(String::new());
    let users = Resource::new(move || {}, move |_| load_hourly_users());
    // let timesheet = Resource::new(current_user, load_timesheet_for);

    Effect::new(move |_| leptos::logging::log!("{:?}", current_user.get()));

    view! {
        <TimeSheetsLayout>
            <Suspense fallback=move || {
                view! { <Loading/> }
            }>
                {move || match users.get() {
                    Some(Ok(a)) => {
                        view! {
                            <div>
                                <label for="user_selected"></label>
                                <select
                                    name="user_selected"
                                    id="user_selected"
                                    on:change=move |e| set_current_user.set(event_target_value(&e))
                                >
                                    <Show when=move || current_user.get().is_empty()>
                                        <option value="">"-- Select User --"</option>
                                    </Show>
                                    {a
                                        .iter()
                                        .map(|user| {
                                            view! {
                                                <option value=user
                                                    .id
                                                    .to_string()>
                                                    {user.last_name.clone()} ", " {user.first_name.clone()}
                                                </option>
                                            }
                                        })
                                        .collect_view()}
                                </select>
                            </div>
                        }
                            .into_any()
                    }
                    _ => view! { <div>"Server Error"</div> }.into_any(),
                }}
                <Show when=move || { !current_user.get().is_empty() }>
                    <p>"Show TimeSheet"</p>

                </Show>

            </Suspense>
        </TimeSheetsLayout>
    }
}

#[component]
pub fn TimeSheetsAdjustment() -> impl IntoView {
    let users = Resource::new(move || {}, move |_| load_hourly_users());
    let create_adjustment = ServerAction::<CreateAdjustment>::new();
    view! {
        <TimeSheetsLayout>
            <ActionForm action=create_adjustment>
                <Suspense fallback=move || {
                    view! { <Loading/> }
                }>
                    {move || match users.get() {
                        Some(Ok(a)) => {
                            view! {
                                <div>
                                    <label for="user_id">"User"</label>
                                    <select name="user_id" id="user_id">
                                        {a
                                            .iter()
                                            .map(|user| {
                                                view! {
                                                    <option value=user
                                                        .id
                                                        .to_string()>
                                                        {user.last_name.clone()} ", " {user.first_name.clone()}
                                                    </option>
                                                }
                                            })
                                            .collect_view()}
                                    </select>
                                </div>
                            }
                                .into_any()
                        }
                        _ => view! { <div>"Server Error"</div> }.into_any(),
                    }}

                </Suspense>
                <div>
                    <label for="date">"Date"</label>
                    <input type="date" name="date" id="date"/>
                </div>
                <div>
                    <label for="hours">"Hours"</label>
                    <input type="number" name="hours" id="hours"/>
                </div>
                <button type="submit">"Submit"</button>
            </ActionForm>
        </TimeSheetsLayout>
    }
}

#[server]
pub async fn create_adjustment(
    user_id: Uuid,
    date: NaiveDate,
    hours: i32,
    response: String,
) -> Result<(), ServerFnError> {
    use crate::models::adjustments::create_adjustment as create_adjustment_fn;

    match create_adjustment_fn(&user_id, date, hours, &response).await {
        Ok(_) => Ok(()),
        Err(_) => Err(ServerFnError::ServerError("Server Error".to_string())),
    }
}

#[component]
pub fn TimeSheetsPending() -> impl IntoView {
    view! {
        <TimeSheetsLayout>
            <h1>"TimeSheets | To Do"</h1>
        </TimeSheetsLayout>
    }
}
