use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum State {
    Inactive = 0,
    Salary = 1,
    Hourly = 2,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub state: i32,
}

#[cfg(feature = "ssr")]
pub struct UserDB {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub state: i32,
}

#[cfg(feature = "ssr")]
use {crate::database::get_db, sqlx::*, uuid::Uuid};

#[cfg(feature = "ssr")]
impl UserDB {
    #[tracing::instrument]
    pub async fn get_all_by_state(state: State) -> Result<Vec<Self>, sqlx::Error> {
        tracing::info!("Fetching all hourly users");
        let db = get_db();
        query_as!(
            UserDB,
            r#"
            SELECT id, last_name, first_name, phone_number, state
            FROM users
            WHERE state = $1;
            "#,
            state as i32
        ).fetch_all(db).await
    }

    #[tracing::instrument]
    pub async fn get(id: Uuid) -> Result<Self, sqlx::Error> {
        let db = get_db();
        query_as!(
            UserDB,
            r#"
SELECT
    id,
    last_name,
    first_name,
    phone_number,
    state
FROM
    users
WHERE
    id = $1;
            "#,
            id
        )
        .fetch_one(db)
        .await
    }
}

#[cfg(feature = "ssr")]
impl User {
    #[tracing::instrument]
    pub async fn update(&self) -> Result<Self, sqlx::Error> {
        let db = get_db();

        let id = uuid::Uuid::parse_str(&self.id).expect("Invalid UUID");

        query_as!(
            User,
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
            id
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
            User,
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
    pub id: String,
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

#[cfg(feature = "ssr")]
impl From<UserDB> for User {
    fn from(user: UserDB) -> Self {
        Self {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            phone_number: user.phone_number,
            state: user.state,
        }
    }
}
