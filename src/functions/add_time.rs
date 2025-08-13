use crate::Result;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddTimeInput {
    pub date: String,
    pub time: String,
    pub reason: String,
}

#[server]
pub async fn add_time(input: AddTimeInput) -> Result<()> {
    use super::current_user;
    use crate::models::{CorrectionState, TimeLogDB};
    use chrono::{DateTime, Utc};
    use tracing::error;

    let Some((user_id, _)) = current_user().await else {
        error!("User not found");
        return Err(crate::Error::Unauthorized);
    };

    let Ok(event_time) = DateTime::parse_from_rfc3339(&input.time) else {
        error!("Parsing Error");
        return Err(crate::Error::InternalError);
    };

    let event_time = event_time.with_timezone(&Utc);

    match TimeLogDB::add_correction(user_id, event_time, input.reason, CorrectionState::Pending)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(crate::Error::Db(format!(
            "Database add correction error: {e}"
        ))),
    }
}
