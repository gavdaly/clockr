#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TimeLog {
    pub id: String,
    pub event_time: i64,
    pub correction: Option<Correction>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Correction {
    pub id: String,
    pub reason: String,
    pub state: u16,
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
use uuid::Uuid;

#[cfg(feature = "ssr")]
impl TimeLogDB {
    pub async fn add(user_id: Uuid) -> Result<(), sqlx::Error> {
        use crate::database::get_db;
        use uuid::Uuid;
        let db = get_db();
        let id = Uuid::from_bytes(ulid::Ulid::new().to_bytes());

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

impl Correction {
    pub fn from_options(
        id: Option<String>,
        reason: Option<String>,
        state: Option<u16>,
    ) -> Option<Self> {
        if id.is_none() || reason.is_none() || state.is_none() {
            return None;
        }

        Some(Self {
            id: id.unwrap_or("".to_string()),
            reason: reason.unwrap_or("".to_string()),
            state: state.unwrap_or(0),
        })
    }
}
