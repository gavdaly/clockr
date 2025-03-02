#[cfg(feature = "ssr")]
use tracing::{error, info};
use uuid::Uuid;

pub struct MagicLink {
    pub id: Uuid,
    pub user_id: Uuid,
}

#[cfg(feature = "ssr")]
use {crate::database::get_db, sqlx::query_as};

#[cfg(feature = "ssr")]
impl MagicLink {
    /// Creates a new magic link for a given user ID
    ///
    /// # Arguments
    /// * `user_id` - UUID of the user to create the magic link for
    ///
    /// # Returns
    /// * `Ok(Uuid)` - The ID of the created magic link
    /// * `Err(sqlx::Error)` - If there was an error inserting into the database
    #[tracing::instrument]
    pub async fn create(user_id: Uuid) -> Result<Uuid, sqlx::Error> {
        info!("Creating magic link for user {}", user_id);
        let db = get_db();
        let link = query_as!(
            MagicLink,
            "INSERT
        INTO magic_links
            (user_id)
        VALUES
            ($1)
        RETURNING id, user_id;",
            user_id
        )
        .fetch_one(db)
        .await?;

        Ok(link.id)
    }

    /// Retrieves the user ID associated with a magic link
    ///
    /// # Arguments
    /// * `id` - String UUID of the magic link to look up
    ///
    /// # Returns
    /// * `Ok(Uuid)` - The user ID associated with the magic link
    /// * `Err(sqlx::Error)` - If the magic link is invalid or not found
    #[tracing::instrument]
    pub async fn get(id: &str) -> Result<Uuid, sqlx::Error> {
        info!("Getting magic link for id {}", id);
        let Ok(id) = Uuid::parse_str(id) else {
            error!("invalid id");
            return Err(sqlx::Error::RowNotFound);
        };
        let link = query_as!(
            MagicLink,
            "SELECT id, user_id FROM magic_links WHERE id = $1",
            id
        )
        .fetch_one(get_db())
        .await
        .map_err(|e| {
            error!("failed to fetch magic link: {}", e);
            e
        })?;
        Ok(link.user_id)
    }
}
