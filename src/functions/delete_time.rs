use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteTimeInput {
    pub time_log_id: String,
}

#[server]
#[tracing::instrument]
pub async fn delete_time(input: DeleteTimeInput) -> Result<(), ServerFnError> {
    use super::current_user;
    use crate::models::{TimeLogDB, CorrectionState};
    use leptos::prelude::server_fn::error::*;
    use server_fn::error::NoCustomError;

    let _user_id = current_user()
       .await?;

    let time_log_id = Uuid::parse_str(&input.time_log_id)
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    TimeLogDB::update_correction(
        time_log_id,
        CorrectionState::UserDeleted,
    )
    .await
    .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    Ok(())
}