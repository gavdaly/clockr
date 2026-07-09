#[cfg(feature = "ssr")]
use {
    crate::database::get_db,
    chrono::{DateTime, Duration, Utc},
    sqlx::Row,
    tracing::{error, info},
    uuid::Uuid,
};

#[derive(Clone, Copy, Debug)]
pub enum MagicLinkPurpose {
    SignIn,
    Invite,
    Recovery,
}

impl MagicLinkPurpose {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SignIn => "sign_in",
            Self::Invite => "invite",
            Self::Recovery => "recovery",
        }
    }
}

#[cfg(feature = "ssr")]
pub struct MagicLink {
    pub id: Uuid,
    pub user_id: Uuid,
    pub purpose: String,
    pub expires_at: DateTime<Utc>,
    pub consumed_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
impl MagicLink {
    #[tracing::instrument]
    pub async fn create(user_id: Uuid) -> Result<Uuid, sqlx::Error> {
        Self::create_for(user_id, MagicLinkPurpose::SignIn, Duration::days(7)).await
    }

    #[tracing::instrument]
    pub async fn create_for(
        user_id: Uuid,
        purpose: MagicLinkPurpose,
        ttl: Duration,
    ) -> Result<Uuid, sqlx::Error> {
        info!(
            "Creating {} magic link for user {}",
            purpose.as_str(),
            user_id
        );

        let expires_at = Utc::now() + ttl;
        let link = sqlx::query(
            r#"
INSERT INTO magic_links (user_id, purpose, expires_at)
VALUES ($1, $2, $3)
RETURNING id, user_id, purpose, expires_at, consumed_at;
            "#,
        )
        .bind(user_id)
        .bind(purpose.as_str())
        .bind(expires_at)
        .fetch_one(get_db())
        .await?;

        Ok(Self::from_row(&link).id)
    }

    #[tracing::instrument]
    pub async fn consume(id: &str) -> Result<Self, sqlx::Error> {
        info!("Consuming magic link for id {}", id);
        let Ok(id) = Uuid::parse_str(id) else {
            error!("invalid id");
            return Err(sqlx::Error::RowNotFound);
        };

        let link = sqlx::query(
            r#"
UPDATE magic_links
SET consumed_at = NOW(), updated_at = NOW()
WHERE id = $1
  AND consumed_at IS NULL
  AND expires_at > NOW()
RETURNING id, user_id, purpose, expires_at, consumed_at;
            "#,
        )
        .bind(id)
        .fetch_one(get_db())
        .await
        .map_err(|error| {
            error!("failed to consume magic link: {}", error);
            error
        })?;

        Ok(Self::from_row(&link))
    }

    #[tracing::instrument]
    pub async fn get(id: &str) -> Result<Uuid, sqlx::Error> {
        Self::consume(id).await.map(|link| link.user_id)
    }

    fn from_row(row: &sqlx::postgres::PgRow) -> Self {
        Self {
            id: row.get("id"),
            user_id: row.get("user_id"),
            purpose: row.get("purpose"),
            expires_at: row.get("expires_at"),
            consumed_at: row.get("consumed_at"),
        }
    }
}
