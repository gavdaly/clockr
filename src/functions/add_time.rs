use leptos::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddTimeInput {
    pub date: String,
    pub time: String,
    pub reason: String,
}

#[server]
#[tracing::instrument]
pub async fn add_time(input: AddTimeInput) -> Result<(), ServerFnError> {
    use crate::models::{TimeLogDB, CorrectionState};
    use axum::extract::Extension;
    use axum_session::Session;
    use axum_session_sqlx::SessionPgPool;
    use leptos::prelude::server_fn::error::*;
    use leptos_axum::extract;
    use server_fn::error::NoCustomError;

    let session: Extension<Session<SessionPgPool>> = extract().await?;

    let Some(id) = session.get::<String>("id") else {
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "User not authenticated".into(),
        ));
    };

    let user_id = Uuid::parse_str(&id).expect("Should be valid uuid");
    let event_time = DateTime::parse_from_rfc3339(&input.time)
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?
        .with_timezone(&Utc);

    TimeLogDB::add_correction(
        user_id,
        event_time,
        input.reason,
        CorrectionState::Pending,
    )
    .await
    .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    Ok(())
}