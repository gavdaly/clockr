use chrono::{DateTime, Utc};
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
    pub email: Option<String>,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub state: i32,
}

#[cfg(feature = "ssr")]
pub struct UserDB {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub email: Option<String>,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub state: i32,
}

#[cfg(feature = "ssr")]
use {crate::database::get_db, sqlx::*, uuid::Uuid};

#[cfg(feature = "ssr")]
impl UserDB {
    #[tracing::instrument]
    pub async fn get_all() -> Result<Vec<Self>, sqlx::Error> {
        tracing::info!("Fetching all users");
        let db = get_db();
        query_as!(
            UserDB,
            r#"
            SELECT id, last_name, first_name, phone_number, email, email_verified_at, state
            FROM users
            ORDER BY last_name ASC, first_name ASC;
            "#
        )
        .fetch_all(db)
        .await
    }

    #[tracing::instrument]
    pub async fn get_all_by_state(state: State) -> Result<Vec<Self>, sqlx::Error> {
        tracing::info!("Fetching all hourly users");
        let db = get_db();
        query_as!(
            UserDB,
            r#"
            SELECT id, last_name, first_name, phone_number, email, email_verified_at, state
            FROM users
            WHERE state = $1;
            "#,
            state as i32
        )
        .fetch_all(db)
        .await
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
    email,
    email_verified_at,
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
SET first_name = $1,
    last_name = $2,
    phone_number = $3,
    email = $4,
    email_verified_at = CASE
        WHEN email IS NOT DISTINCT FROM $4 THEN email_verified_at
        ELSE NULL
    END,
    state = $5,
    updated_at = NOW()
WHERE id = $6
RETURNING first_name, last_name, phone_number, email, email_verified_at, state, id
"#,
            self.first_name,
            self.last_name,
            self.phone_number,
            self.email,
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
        email: Option<String>,
        state: i32,
    ) -> Result<Self, sqlx::Error> {
        let db = get_db();
        query_as!(
            User,
            r#"
INSERT INTO users(first_name, last_name, phone_number, email, state)
VALUES ($1, $2, $3, $4, $5)
RETURNING id, first_name, last_name, phone_number, email, email_verified_at, state
        "#,
            first_name,
            last_name,
            phone_number,
            email,
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
#[tracing::instrument(skip(email), fields(email = %obfuscate_email(email)))]
pub async fn store_user_email(user_id: Uuid, email: &str) -> Result<(), sqlx::Error> {
    let db = get_db();

    sqlx::query(
        r#"
UPDATE users
SET email = $1,
    email_verified_at = CASE
        WHEN email = $1 THEN email_verified_at
        ELSE NULL
    END,
    updated_at = NOW()
WHERE id = $2
  AND (email IS NULL OR email = $1);
        "#,
    )
    .bind(email)
    .bind(user_id)
    .execute(db)
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
#[tracing::instrument(skip(email), fields(email = %obfuscate_email(email)))]
pub async fn mark_user_email_verified(user_id: Uuid, email: &str) -> Result<(), sqlx::Error> {
    let db = get_db();

    let result = sqlx::query(
        r#"
UPDATE users
SET email = $1,
    email_verified_at = NOW(),
    updated_at = NOW()
WHERE id = $2
  AND (email IS NULL OR email = $1);
        "#,
    )
    .bind(email)
    .bind(user_id)
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}

#[cfg(feature = "ssr")]
fn obfuscate_email(email: &str) -> String {
    let Some((local, domain)) = email.split_once('@') else {
        return "[invalid-email]".to_string();
    };

    let visible_local: String = local.chars().take(1).collect();
    let visible_domain: String = domain.chars().take(1).collect();

    format!("{visible_local}***@{visible_domain}***")
}

#[cfg(feature = "ssr")]
impl From<UserDB> for User {
    fn from(user: UserDB) -> Self {
        Self {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            phone_number: user.phone_number,
            email: user.email,
            email_verified_at: user.email_verified_at,
            state: user.state,
        }
    }
}
