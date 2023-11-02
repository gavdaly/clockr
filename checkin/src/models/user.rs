use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub provider: Option<i32>,
    pub phone_number: String,
    pub display_name: Option<String>,
    pub api_id: Option<i32>,
    pub state: State,
    pub settings: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum State {
    Inactive = 0,
    Salary = 1,
    Hourly = 2,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub state: i32,
    pub check_in: Option<DateTime<Utc>>,
    pub checked_in: Option<bool>,
}

#[cfg(feature = "ssr")]
use sqlx::*;

#[cfg(feature = "ssr")]
impl UserPublic {
    pub async fn get_all_hourly() -> Result<Vec<Self>, sqlx::Error> {
        use crate::database;
        let db = database::get_db();
        query_as!(UserPublic, r#"
SELECT
	u.id, last_name, first_name, phone_number, u.state, start_time as check_in, (end_time IS NULL) as checked_in
FROM
	users u
LEFT JOIN sessions s
ON u.id = s.user_id
WHERE
	u.state = 2 AND s.end_time IS NULL
ORDER BY last_name, first_name;
                            "#).fetch_all(db).await
    }

    pub async fn get(id: Uuid) -> Result<Self, sqlx::Error> {
        use crate::database;
        let db = database::get_db();
        query_as!(UserPublic, r#"
SELECT
    u.id, last_name, first_name, phone_number, u.state, (end_time IS NULL) as checked_in, start_time as check_in
FROM
    users u
LEFT JOIN sessions s
ON u.id = s.user_id
WHERE
    u.id = $1
ORDER BY start_time DESC
LIMIT 1
                "#, id).fetch_one(db).await
    }

    pub async fn get_phone(phone: &str) -> Result<Self, sqlx::Error> {
        use crate::database;
        leptos::tracing::info!("-- Getting Phone Numeber: {}", phone);

        let db = database::get_db();
        query_as!(UserPublic, r#"
SELECT
	u.id, last_name, first_name, phone_number, u.state, start_time as check_in, (end_time IS NULL) as checked_in
FROM
	users u
LEFT JOIN sessions s
ON u.id = s.user_id
WHERE
	phone_number = $1;
	       "#, phone).fetch_one(db).await
    }
}
