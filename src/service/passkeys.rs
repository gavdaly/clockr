#[cfg(feature = "ssr")]
use {
    std::sync::{Arc, OnceLock},
    url::Url,
    webauthn_rs::prelude::*,
};

#[cfg(feature = "ssr")]
static WEBAUTHN: OnceLock<Arc<Webauthn>> = OnceLock::new();

#[cfg(feature = "ssr")]
pub fn get_webauthn() -> Result<Arc<Webauthn>, WebauthnError> {
    if let Some(webauthn) = WEBAUTHN.get() {
        return Ok(webauthn.clone());
    }

    let rp_id = std::env::var("WEBAUTHN_RP_ID").unwrap_or_else(|_| "localhost".to_string());
    let rp_origin =
        std::env::var("WEBAUTHN_RP_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let rp_name = std::env::var("WEBAUTHN_RP_NAME").unwrap_or_else(|_| "Clockr".to_string());
    let rp_origin = Url::parse(&rp_origin).map_err(|_| WebauthnError::Configuration)?;
    let webauthn = WebauthnBuilder::new(&rp_id, &rp_origin)?
        .rp_name(&rp_name)
        .build()?;
    let webauthn = Arc::new(webauthn);

    let _ = WEBAUTHN.set(webauthn.clone());
    Ok(webauthn)
}
