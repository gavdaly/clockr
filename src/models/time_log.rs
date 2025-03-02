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
use ulid::Ulid;

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

#[derive(Debug)]
pub(crate) struct TimeLogDB {
    id: String,
    user_id: String,
    event_time: i64,
    location_latitude: Option<f64>,
    location_longitude: Option<f64>,
    mac_address: Option<String>,
    correction: Option<Correction>,
}

#[cfg(feature = "ssr")]
impl TimeLogDB {
    pub async fn add(user_id: uuid::Uuid) -> Result<(), sqlx::Error> {
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
}

#[cfg(feature = "ssr")]
impl CorrectionDB {
    pub fn from_options(
        reason: Option<String>,
        state: Option<i16>,
        time_log_id: Option<Ulid>,
    ) -> Option<Self> {
        let Some(reason) = reason else {
            return None;
        };
        let Some(state) = state else {
            return None;
        };
        let Some(time_log_id) = time_log_id else {
            return None;
        };

        Some(Self { reason, state, time_log_id })
    }
}

#[cfg(feature = "ssr")]
impl From <CorrectionDB> for Correction {
    fn from(c: CorrectionDB) -> Self {
        Self {
            reason: c.reason,
            state: c.state,
        }
    }
}
