#[cfg(feature = "ssr")]
use {
    crate::database::get_db,
    chrono::{DateTime, Duration, Utc},
    serde_json::Value,
    sqlx::Row,
    uuid::Uuid,
};

#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
pub struct PasskeyRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub credential_id: Vec<u8>,
    pub passkey: Value,
    pub label: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
pub struct WebauthnChallenge {
    pub id: Uuid,
    pub user_id: Uuid,
    pub state: Value,
}

#[cfg(feature = "ssr")]
pub async fn insert_passkey(
    user_id: Uuid,
    credential_id: Vec<u8>,
    passkey: Value,
    label: Option<String>,
) -> Result<Uuid, sqlx::Error> {
    let id = sqlx::query(
        r#"
        INSERT INTO user_passkeys(user_id, credential_id, passkey, label)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
    )
    .bind(user_id)
    .bind(credential_id)
    .bind(passkey)
    .bind(label)
    .fetch_one(get_db())
    .await?
    .try_get("id")?;

    Ok(id)
}

#[cfg(feature = "ssr")]
pub async fn list_user_passkeys(user_id: Uuid) -> Result<Vec<PasskeyRecord>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT id, user_id, credential_id, passkey, label, created_at, last_used_at
        FROM user_passkeys
        WHERE user_id = $1
        ORDER BY created_at ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(get_db())
    .await?;

    rows.into_iter()
        .map(|row| {
            Ok(PasskeyRecord {
                id: row.try_get("id")?,
                user_id: row.try_get("user_id")?,
                credential_id: row.try_get("credential_id")?,
                passkey: row.try_get("passkey")?,
                label: row.try_get("label")?,
                created_at: row.try_get("created_at")?,
                last_used_at: row.try_get("last_used_at")?,
            })
        })
        .collect()
}

#[cfg(feature = "ssr")]
pub async fn load_passkey_by_credential_id(
    credential_id: &[u8],
) -> Result<PasskeyRecord, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT id, user_id, credential_id, passkey, label, created_at, last_used_at
        FROM user_passkeys
        WHERE credential_id = $1
        "#,
    )
    .bind(credential_id)
    .fetch_one(get_db())
    .await?;

    Ok(PasskeyRecord {
        id: row.try_get("id")?,
        user_id: row.try_get("user_id")?,
        credential_id: row.try_get("credential_id")?,
        passkey: row.try_get("passkey")?,
        label: row.try_get("label")?,
        created_at: row.try_get("created_at")?,
        last_used_at: row.try_get("last_used_at")?,
    })
}

#[cfg(feature = "ssr")]
pub async fn update_passkey_after_authentication(
    credential_id: &[u8],
    passkey: Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE user_passkeys
        SET passkey = $2,
            last_used_at = NOW()
        WHERE credential_id = $1
        "#,
    )
    .bind(credential_id)
    .bind(passkey)
    .execute(get_db())
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn insert_challenge(
    user_id: Uuid,
    flow: &str,
    state: Value,
) -> Result<Uuid, sqlx::Error> {
    let expires_at = Utc::now() + Duration::minutes(5);
    let id = sqlx::query(
        r#"
        INSERT INTO webauthn_challenges(user_id, flow, state, expires_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
    )
    .bind(user_id)
    .bind(flow)
    .bind(state)
    .bind(expires_at)
    .fetch_one(get_db())
    .await?
    .try_get("id")?;

    Ok(id)
}

#[cfg(feature = "ssr")]
pub async fn load_user_challenge(
    id: Uuid,
    user_id: Uuid,
    flow: &str,
) -> Result<WebauthnChallenge, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT id, user_id, state
        FROM webauthn_challenges
        WHERE id = $1
          AND user_id = $2
          AND flow = $3
          AND consumed_at IS NULL
          AND expires_at > NOW()
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(flow)
    .fetch_one(get_db())
    .await?;

    Ok(WebauthnChallenge {
        id: row.try_get("id")?,
        user_id: row.try_get("user_id")?,
        state: row.try_get("state")?,
    })
}

#[cfg(feature = "ssr")]
pub async fn load_challenge(id: Uuid, flow: &str) -> Result<WebauthnChallenge, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT id, user_id, state
        FROM webauthn_challenges
        WHERE id = $1
          AND flow = $2
          AND consumed_at IS NULL
          AND expires_at > NOW()
        "#,
    )
    .bind(id)
    .bind(flow)
    .fetch_one(get_db())
    .await?;

    Ok(WebauthnChallenge {
        id: row.try_get("id")?,
        user_id: row.try_get("user_id")?,
        state: row.try_get("state")?,
    })
}

#[cfg(feature = "ssr")]
pub async fn consume_challenge(id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE webauthn_challenges
        SET consumed_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(get_db())
    .await?;

    Ok(())
}
