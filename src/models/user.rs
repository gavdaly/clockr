use chrono::{DateTime, Utc};
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// pub struct UserToday {
//     pub id: Uuid,
//     pub first_name: String,
//     pub last_name: String,
//     pub state: State,
//     pub check_ins: Vec<DateTime<Utc>>,
//     pub week_duration: u64,
//     pub previous_day_possible_errors: u16,
// }

#[derive(Clone, Debug, Default, Store, Deserialize, Serialize)]
pub enum CurrentUser {
    Authenticated(UserToday),
    #[default]
    Guest,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum State {
    Inactive = 0,
    Salary = 1,
    Hourly = 2,
}

#[derive(Clone, Debug, Store, Deserialize, Serialize)]
pub struct UserToday {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub state: i32,
    pub check_in: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserUpdate {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub state: i32,
}

#[cfg(feature = "ssr")]
use {crate::database::get_db, sqlx::*};

#[cfg(feature = "ssr")]
impl UserToday {
    #[tracing::instrument]
    pub async fn get_all_hourly() -> Result<Vec<Self>, sqlx::Error> {
        tracing::info!("Fetching all hourly users");
        let db = get_db();
        query_as!(
            UserToday,
            r#"
SELECT
    u.id,
    u.last_name,
    u.first_name,
    u.phone_number,
    u.state,
    s.check_in
FROM
    users u
LEFT JOIN (
    SELECT
        user_id,
        MAX(start_time) AS check_in
    FROM
        sessions
    WHERE
        end_time IS NULL
        AND start_time >= CURRENT_DATE -- Ensuring the session started today
    GROUP BY
        user_id
) s ON u.id = s.user_id
WHERE
    u.state = 2 -- Ensuring that the user's state is 2
ORDER BY
    u.last_name ASC,
    u.first_name ASC;"#
        )
        .fetch_all(db)
        .await
    }

    #[tracing::instrument]
    pub async fn get(id: Uuid) -> Result<Self, sqlx::Error> {
        let db = get_db();
        query_as!(
            UserToday,
            r#"
SELECT
    u.id,
    u.last_name,
    u.first_name,
    u.phone_number,
    u.state,
    s.check_in
FROM
    users u
LEFT JOIN (
    SELECT
        user_id,
        MAX(start_time) AS check_in
    FROM
        sessions
    WHERE
        end_time IS NULL
        AND start_time >= CURRENT_DATE -- Ensuring the session started today
    GROUP BY
        user_id
) s ON u.id = s.user_id
WHERE
    u.id = $1;
        "#,
            id
        )
        .fetch_one(db)
        .await
    }
}

#[cfg(feature = "ssr")]
impl UserUpdate {
    #[tracing::instrument]
    pub async fn update(&self) -> Result<Self, sqlx::Error> {
        let db = get_db();
        query_as!(
            UserUpdate,
            r#"
UPDATE users
SET first_name = $1, last_name = $2, phone_number = $3, state = $4
WHERE id = $5
RETURNING first_name, last_name, phone_number, state, id
"#,
            self.first_name,
            self.last_name,
            self.phone_number,
            self.state,
            self.id
        )
        .fetch_one(db)
        .await
    }

    #[tracing::instrument]
    pub async fn insert(
        first_name: &str,
        last_name: &str,
        phone_number: &str,
        state: i32,
    ) -> Result<Self, sqlx::Error> {
        let db = get_db();
        query_as!(
            UserUpdate,
            r#"
INSERT INTO users(first_name, last_name, phone_number, state)
VALUES ($1, $2, $3, $4)
RETURNING id, first_name, last_name, phone_number, state
        "#,
            first_name,
            last_name,
            phone_number,
            state
        )
        .fetch_one(db)
        .await
    }
}

#[cfg(feature = "ssr")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserPhone {
    pub id: Uuid,
    pub phone_number: String,
}

#[cfg(feature = "ssr")]
#[tracing::instrument]
pub async fn get_user_by_phone(phone: &str) -> Result<UserPhone, sqlx::Error> {
    use sqlx::*;
    tracing::info!("Getting user by Phone Numeber: {}", phone);

    let db = get_db();
    let result = query_as!(
        UserPhone,
        r#"
SELECT
    id, phone_number
FROM
    users
WHERE
    phone_number = $1;
       "#,
        phone
    )
    .fetch_one(db)
    .await;

    tracing::info!("Got User: {:?}", result);
    result
}
