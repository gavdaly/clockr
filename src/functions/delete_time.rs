use crate::Result;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteTimeInput {
    pub time_log_id: String,
}

#[server]
#[tracing::instrument]
pub async fn delete_time(input: DeleteTimeInput) -> Result<()> {
    use super::current_user;
    use crate::models::{CorrectionState, TimeLogDB};
    use tracing::error;
    use uuid::Uuid;

    let _user_id = current_user().await.ok_or(crate::Error::Unauthorized);

    let Ok(time_log_id) = Uuid::parse_str(&input.time_log_id) else {
        error!("Error parsing id: {input:?}");
        return Err(crate::Error::InternalError);
    };

    match TimeLogDB::update_correction(time_log_id, CorrectionState::UserDeleted).await {
        Ok(()) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

