use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteTimeInput {
    pub time_log_id: String,
}

#[server]
#[tracing::instrument]
pub async fn delete_time(input: DeleteTimeInput) -> Result<(), ServerFnError> {
    use crate::models::{TimeLogDB, CorrectionState};
    use axum::extract::Extension;
    use axum_session::Session;
    use axum_session_sqlx::SessionPgPool;
    use leptos::prelude::server_fn::error::*;
    use leptos_axum::extract;
    use server_fn::error::NoCustomError;

    let session: Extension<Session<SessionPgPool>> = extract().await?;

    let Some(_) = session.get::<String>("id") else {
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "User not authenticated".into(),
        ));
    };

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