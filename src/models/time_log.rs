#[derive(Clone, Debug, Store, Patch, serde::Serialize, serde::Deserialize)]
pub struct TimeLog {
    pub id: String,
    user_id: String,
    pub event_time: i64,
    location_latitude: Option<f64>,
    location_longitude: Option<f64>,
    mac_address: Option<String>,
    correction: Option<Correction>
}

#[derive(Clone, Debug, Store, Patch, serde::Serialize, serde::Deserialize)]
pub struct Correction {
    id: String,
    reason: String,
    state: u16,
}

#[derive(Clone, Debug, Default, Store, serde::Serialize, serde::Deserialize)]
pub enum CorrectionState {
    #[default]
    Pending = 0,
    Accepted = 1,
    Rejected = 2,
}

impl TimeLog {
    pub fn entry(&self) -> (String, i64) {
        ("8:03 AM".to_string(), self.event_time)
    }
    pub fn new(user_id: String, event_time: i64) -> Self {
        Self {
            id: "".to_string(),
            user_id,
            event_time,
            location_latitude: None,
            location_longitude: None,
            mac_address: None,
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
    correction: Option<Correction>
}

use reactive_stores::{Patch, Store};
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
