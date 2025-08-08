use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddTimeInput {
    pub date: String,
    pub time: String,
    pub reason: String,
}

#[server]
#[tracing::instrument]
pub async fn add_time(input: AddTimeInput) -> Result<(), ServerFnError> {
    use super::current_user;
    use crate::models::{CorrectionState, TimeLogDB};
    use chrono::{DateTime, Utc};
    use leptos::prelude::server_fn::error::*;
    use server_fn::error::NoCustomError;

    let (user_id, _) = current_user().await?;

    let event_time = DateTime::parse_from_rfc3339(&input.time)
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?
        .with_timezone(&Utc);

    TimeLogDB::add_correction(user_id, event_time, input.reason, CorrectionState::Pending)
        .await
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    Ok(())
}
