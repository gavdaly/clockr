use crate::components::session_form::CorrectionForm;
use crate::models::sessions::Session;
use crate::models::time_sheets::TimeSheet;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

/// Renders the home page of your application.
#[component]
pub fn TimeSheetLayout(children: Children) -> impl IntoView {
    view! { <section class="stack">{children()}</section> }
}

#[component]
pub fn TimeSheetDisplay() -> impl IntoView {
    // let timesheet = Resource::new(|| {}, |_| get_active_user_timesheet()).read();
    {
        view! {
            <TimeSheetLayout>
                <section class="stack">
                    <A href="/app/timesheet/missing">"Add missing time"</A>
                    <p>"Time Sheet"</p>
                </section>
            </TimeSheetLayout>
        }
    }
}

#[component]
pub fn TimeSheetMissing() -> impl IntoView {
    view! {
        <TimeSheetLayout>
            <CorrectionForm uuid=None/>
        </TimeSheetLayout>
    }
}

#[derive(Params, Clone, PartialEq)]
struct TimeSheetEditParams {
    uuid: Option<String>,
}

#[component]
pub fn TimeSheetEdit() -> impl IntoView {
    let params = use_params::<TimeSheetEditParams>();
    let session = ServerAction::<GetSession>::new();
    let value = session.value();
    let _date = move || match value.get() {
        Some(Ok(Session { start_time, .. })) => Some(start_time.format("%y-%m-%d").to_string()),
        _ => None,
    };
    match params.get() {
        Ok(TimeSheetEditParams { uuid: Some(uuid) }) => {
            session.dispatch(GetSession { uuid: uuid.clone() });

            view! {
                <TimeSheetLayout>
                    <CorrectionForm uuid=Some(uuid)/>
                </TimeSheetLayout>
            }.into_any()
        }
        Err(e) => view! { <div data-state="error">"Error getting session: " {e.to_string()}</div> }
            .into_any(),
        _ => view! { <div data-state="error">"Coult Not find the user you are looking for!"</div> }
            .into_any(),
    }
}

#[server]
async fn get_session(uuid: String) -> Result<Session, ServerFnError> {
    crate::models::sessions::get_session(&uuid)
        .await
        .map_err(|_| ServerFnError::Request("Error Getting Session".into()))
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
