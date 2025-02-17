#[cfg(feature = "ssr")]
use axum_session::SessionStore;
#[cfg(feature = "ssr")]
use axum_session_sqlx::SessionPgPool;

#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
pub struct UserSession(pub SessionStore<SessionPgPool>);

#[cfg(feature = "ssr")]
impl UserSession {
    pub async fn get_user_id(&self) -> Option<i32> {
        // self.0.get("id").await.ok()
        None
    }
}
