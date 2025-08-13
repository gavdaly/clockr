#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TimeLog {
    pub id: String,
    pub event_time: i64,
    pub correction: Option<Correction>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Correction {
    pub reason: String,
    pub state: i16,
}

#[cfg(feature = "ssr")]
use {
    chrono::{DateTime, Utc},
    ulid::Ulid,
    uuid::Uuid,
};

#[cfg(feature = "ssr")]
pub struct CorrectionDB {
    pub time_log_id: Ulid,
    pub reason: String,
    pub state: i16,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum CorrectionState {
    #[default]
    Pending = 0,
    Accepted = 1,
    Rejected = 2,
    AdminAdded = 10,
    UserDeleted = -1,
    AdminDeleted = -10,
}

impl TimeLog {
    pub fn new(_user_id: String, event_time: i64) -> Self {
        Self {
            id: "".to_string(),
            event_time,
            correction: None,
        }
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug)]
pub struct TimeLogDB;

#[cfg(feature = "ssr")]
#[derive(Debug)]
pub struct SessionRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
impl TimeLogDB {
    pub async fn transform_all_sessions() -> Result<(), sqlx::Error> {
        use crate::database::get_db;

        let db = get_db();
        const BATCH_SIZE: i64 = 100;

        let sessions_exists = sqlx::query!(
            r#"SELECT EXISTS (SELECT 1 FROM sessions) as exists"#
        )
        .fetch_one(db)
        .await?;

        if !sessions_exists.exists.unwrap_or(false) {
            return Ok(());
        }

        loop {
            // Fetch limited batch of sessions
            let sessions = sqlx::query_as!(
                SessionRecord,
                r#"
                SELECT id, user_id, start_time, end_time
                FROM sessions
                LIMIT $1
                "#,
                BATCH_SIZE
            )
            .fetch_all(db)
            .await?;

            // If no sessions left, we're done
            if sessions.is_empty() {
                break;
            }

            for session in sessions {
                let mut tx = db.begin().await?;
                let check_in_id = create_ulid_as_uuid(session.start_time);
                sqlx::query!(
                    r#"
                INSERT INTO time_log (
                    id,
                    user_id,
                    event_time
                ) VALUES (
                    $1,
                    $2,
                    $3
                )"#,
                    check_in_id,
                    session.user_id,
                    session.start_time,
                )
                .execute(&mut *tx)
                .await?;

                if let Some(end_time) = session.end_time {
                    let check_out_id = create_ulid_as_uuid(end_time);
                    sqlx::query!(
                        r#"
                    INSERT INTO time_log (
                        id,
                        user_id,
                        event_time
                    ) VALUES (
                        $1,
                        $2,
                        $3
                    )"#,
                        check_out_id,
                        session.user_id,
                        end_time,
                    )
                    .execute(&mut *tx)
                    .await?;
                }

                sqlx::query!(
                    r#"
                    DELETE FROM sessions
                    WHERE id = $1"#,
                    session.id,
                )
                .execute(&mut *tx)
                .await?;

                tx.commit().await?;
            }
        }

        Ok(())
    }
    pub async fn add(user_id: Uuid) -> Result<(), sqlx::Error> {
        use crate::database::get_db;
        use ulid::Ulid;
        use uuid::Uuid;

        let db = get_db();
        let id = Uuid::from_bytes(Ulid::new().to_bytes());

        sqlx::query!(
            r#"
            INSERT INTO time_log (
                id,
                user_id,
                event_time
            ) VALUES (
                $1,
                $2,
                NOW()
            )"#,
            id,
            user_id,
        )
        .execute(db)
        .await?;

        Ok(())
    }
    pub async fn add_correction(
        user_id: uuid::Uuid,
        event_time: DateTime<Utc>,
        reason: String,
        state: CorrectionState,
    ) -> Result<(), sqlx::Error> {
        use crate::database::get_db;
        let db = get_db();

        let id = Ulid::from_datetime(event_time.into());
        let uuid_id = Uuid::from_bytes(id.to_bytes());

        let mut tx = db.begin().await?;

        sqlx::query!(
            r#"
            INSERT INTO time_log (
                id,
                user_id,
                event_time
            ) VALUES (
                $1,
                $2,
                $3
            )
            RETURNING id, user_id, event_time"#,
            uuid_id,
            user_id,
            event_time,
        )
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO time_log_correction (
                time_log_id,
                reason,
                state
            ) VALUES (
                $1,
                $2,
                $3
            )"#,
            uuid_id,
            reason,
            state.clone() as i16,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn update_correction(
        time_log_id: Uuid,
        state: CorrectionState,
    ) -> Result<(), sqlx::Error> {
        use crate::database::get_db;
        let mut tx = get_db().begin().await?;

        sqlx::query!(
            r#"
            UPDATE time_log_correction
            SET state = $2
            WHERE time_log_id = $1"#,
            time_log_id,
            state.clone() as i16,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}

#[cfg(feature = "ssr")]
fn create_ulid_as_uuid(dt: DateTime<Utc>) -> Uuid {
    let ulid = Ulid::from_datetime(dt.into());
    Uuid::from_bytes(ulid.to_bytes())
}

#[cfg(feature = "ssr")]
impl CorrectionDB {
    pub fn from_options(
        reason: Option<String>,
        state: Option<i16>,
        time_log_id: Option<Ulid>,
    ) -> Option<Self> {
        let reason = reason?;
        let state = state?;
        let time_log_id = time_log_id?;

        Some(Self {
            reason,
            state,
            time_log_id,
        })
    }
}

#[cfg(feature = "ssr")]
impl From<CorrectionDB> for Correction {
    fn from(c: CorrectionDB) -> Self {
        Self {
            reason: c.reason,
            state: c.state,
        }
    }
}

