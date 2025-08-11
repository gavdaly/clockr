use uuid::Uuid;

pub struct MagicLink {
    pub id: Uuid,
    pub user_id: Uuid,
}

#[cfg(feature = "ssr")]
use {crate::database::get_db, sqlx::query_as};

#[cfg(feature = "ssr")]
impl MagicLink {
    #[tracing::instrument]
    pub async fn create(user_id: Uuid) -> Result<Uuid, sqlx::Error> {
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
    #[tracing::instrument]
    pub async fn get(id: &str) -> Result<Uuid, sqlx::Error> {
        let Ok(id) = Uuid::parse_str(id) else {
            panic!("invalid id")
        };
        let db = get_db();
        let link = query_as!(
            MagicLink,
            "SELECT id, user_id FROM magic_links WHERE id = $1",
            id
        )
        .fetch_one(db)
        .await?;
        Ok(link.user_id)
    }
}
