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

#[cfg(feature = "hydrate")]
pub async fn create_passkey_credential(
    public_key: serde_json::Value,
) -> Result<serde_json::Value, String> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use webauthn_rs_proto::{CreationChallengeResponse, RegisterPublicKeyCredential};

    let challenge = serde_json::json!({ "publicKey": public_key });
    let challenge: CreationChallengeResponse =
        serde_json::from_value(challenge).map_err(|e| e.to_string())?;
    let options: web_sys::CredentialCreationOptions = challenge.into();

    let window = web_sys::window().ok_or("Browser window is unavailable")?;
    let promise = window
        .navigator()
        .credentials()
        .create_with_options(&options)
        .map_err(js_error)?;
    let credential = JsFuture::from(promise).await.map_err(js_error)?;
    let credential: web_sys::PublicKeyCredential = credential
        .dyn_into()
        .map_err(|_| "Expected a public key credential")?;
    let credential: RegisterPublicKeyCredential = credential.into();

    serde_json::to_value(credential).map_err(|e| e.to_string())
}

#[cfg(feature = "hydrate")]
pub async fn get_passkey_credential(
    public_key: serde_json::Value,
) -> Result<serde_json::Value, String> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use webauthn_rs_proto::{PublicKeyCredential, RequestChallengeResponse};

    let challenge = serde_json::json!({ "publicKey": public_key });
    let challenge: RequestChallengeResponse =
        serde_json::from_value(challenge).map_err(|e| e.to_string())?;
    let options: web_sys::CredentialRequestOptions = challenge.into();

    let window = web_sys::window().ok_or("Browser window is unavailable")?;
    let promise = window
        .navigator()
        .credentials()
        .get_with_options(&options)
        .map_err(js_error)?;
    let credential = JsFuture::from(promise).await.map_err(js_error)?;
    let credential: web_sys::PublicKeyCredential = credential
        .dyn_into()
        .map_err(|_| "Expected a public key credential")?;
    let credential: PublicKeyCredential = credential.into();

    serde_json::to_value(credential).map_err(|e| e.to_string())
}

#[cfg(feature = "hydrate")]
fn js_error(value: wasm_bindgen::JsValue) -> String {
    value
        .as_string()
        .or_else(|| {
            js_sys::Reflect::get(&value, &"message".into())
                .ok()
                .and_then(|message| message.as_string())
        })
        .unwrap_or_else(|| "Browser credential request failed".to_string())
}
