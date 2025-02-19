use crate::models::corrections::Correction;
use crate::models::sessions::SessionAndCorrection;
use crate::utils::miliseconds_to_string;
use chrono::Local;
use leptos::form::ActionForm;
use leptos::prelude::*;
use leptos_router::components::A;
use uuid::Uuid;

/// Renders a session component that displays session information.
///
/// # Arguments
///
/// * `session` - A reference to a `SessionAndCorrection` struct that contains session data.
#[component]
pub fn Session(session: SessionAndCorrection) -> impl IntoView {
    let id = session.id.to_string();
    let start_time = session.start_time.with_timezone(&Local);
    let start_string = start_time.format("%I:%M %P").to_string();
    let start_dt = start_time.to_string();
    let end_time = session.end_time.map(|t| t.with_timezone(&Local));
    view! {
        <span class="start_time" data-id=id.clone()>
            <time datetime=start_dt>{start_string}</time>
        </span>
        {match end_time {
            Some(t) => {
                view! {
                    <span class="end_time">
                        <time datetime=t.to_string()>{t.format("%I:%M %P").to_string()}</time>
                    </span>

                    <span class="duration">
                        {miliseconds_to_string(&(t - start_time).num_milliseconds()).to_string()}
                    </span>

                    {match session.state {
                        0 => view! { <span class="state">"open"</span> }.into_any(),
                        1 => {
                            view! { <A href=format!("/app/timesheet/edit/{}", id)>edit</A> }
                                .into_any()
                        }
                        2 => {
                            view! { <A href=format!("/app/timesheet/edit/{}", id)>error</A> }
                                .into_any()
                        }
                        3 => view! { <span class="state">"pending"</span> }.into_any(),
                        4 => view! { <span class="state">"accepted"</span> }.into_any(),
                        5 => view! { <span class="state">"rejected"</span> }.into_any(),
                        6 => view! { <span class="state">"done"</span> }.into_any(),
                        _ => {
                            view! {
                                <span class="state" data-state="error">
                                    "ERROR"
                                </span>
                            }
                                .into_any()
                        }
                    }}
                }
                    .into_any()
            }
            None => view! { <span class="open">"Session not closed yet!"</span> }.into_any(),
        }}

        {match session.correction.clone() {
            Some(correction) => {
                view! { <Correction correction session_state=session.state/> }.into_any()
            }
            None => view! {}.into_any(),
        }}
    }
}

#[component]
fn Correction(correction: Correction, session_state: i32) -> impl IntoView {
    let start = correction
        .new_start_time
        .with_timezone(&Local)
        .format("%I:%M %P")
        .to_string();
    let end = correction
        .new_end_time
        .with_timezone(&Local)
        .format("%I:%M %P")
        .to_string();
    let handle_correction_response = ServerAction::<HandleCorrectionResponse>::new();
    match session_state {
        3 => view! {
            <span>{start}</span>
            <span>{end}</span>
            <span>"pending time"</span>
            <span></span>
            <span>"reason"</span>
            <span class="reason">{correction.reason}</span>
            <ActionForm action=handle_correction_response>
                <input type="hidden" name="id" value=correction.id.to_string()/>
                <div>
                    <label for="response">"response"</label>
                    <textarea id="response" name="response"></textarea>
                </div>
                <fieldset>
                    <div>
                        <legend>"Response Status"</legend>
                        <input type="radio" id="accepted" name="status" value="4" checked=true/>
                        <label for="accepted">"accepted"</label>
                    </div>
                    <div>
                        <input type="radio" id="rejected" name="status" value="5"/>
                        <label for="rejected">"rejected"</label>
                    </div>
                </fieldset>
                <button type="submit">"submit"</button>
            </ActionForm>
        }
        .into_any(),
        4 | 5 => view! {
            <span>"response"</span>
            <span class="reason">{correction.response}</span>
        }
        .into_any(),
        _ => view! {}.into_any(),
    }
}

#[server]
async fn handle_correction_response(
    response: String,
    status: u32,
    id: Uuid,
) -> Result<(), ServerFnError> {
    use crate::models::corrections::correction_response;
    use crate::models::user::UserToday;
    use axum_session::SessionAnySession;
    use leptos::prelude::server_fn::error::*;

    let session = use_context::<SessionAnySession>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Session missing.".into()))?;
    let user_id = session.get::<Uuid>("id").ok_or_else(|| {
        ServerFnError::<NoCustomError>::ServerError("Error getting Session!".into())
    })?;
    let user = UserToday::get(user_id).await?;

    if user.state != 1 {
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "User not authorized!".into(),
        ));
    }

    match correction_response(id, status, &response).await {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}
