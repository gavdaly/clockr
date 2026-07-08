use crate::Result;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PasskeyChallenge {
    pub challenge_id: Uuid,
    pub public_key: Value,
}

#[server]
pub async fn start_passkey_registration() -> Result<PasskeyChallenge> {
    use crate::models::passkey::insert_challenge;
    use crate::models::user::UserDB;
    use crate::service::passkeys::get_webauthn;

    let (user_id, _) = super::current_user()
        .await
        .ok_or(crate::Error::Unauthorized)?;
    let user = UserDB::get(user_id).await?;
    let webauthn = get_webauthn().map_err(|_| crate::Error::InternalError)?;
    let display_name = format!("{} {}", user.first_name, user.last_name);

    let (public_key, state) = webauthn
        .start_passkey_registration(user_id, &user.phone_number, &display_name, None)
        .map_err(|_| crate::Error::InternalError)?;
    let state = serde_json::to_value(state).map_err(|_| crate::Error::InternalError)?;
    let public_key = serde_json::to_value(public_key).map_err(|_| crate::Error::InternalError)?;
    let challenge_id = insert_challenge(user_id, "registration", state).await?;

    Ok(PasskeyChallenge {
        challenge_id,
        public_key,
    })
}

#[server]
pub async fn finish_passkey_registration(
    challenge_id: Uuid,
    credential: Value,
    label: Option<String>,
) -> Result<()> {
    use crate::models::passkey::{consume_challenge, insert_passkey, load_user_challenge};
    use crate::service::passkeys::get_webauthn;
    use webauthn_rs::prelude::*;

    let (user_id, _) = super::current_user()
        .await
        .ok_or(crate::Error::Unauthorized)?;
    let challenge = load_user_challenge(challenge_id, user_id, "registration").await?;
    let state: PasskeyRegistration =
        serde_json::from_value(challenge.state).map_err(|_| crate::Error::InternalError)?;
    let credential: RegisterPublicKeyCredential =
        serde_json::from_value(credential).map_err(|_| crate::Error::InternalError)?;
    let webauthn = get_webauthn().map_err(|_| crate::Error::InternalError)?;
    let passkey = webauthn
        .finish_passkey_registration(&credential, &state)
        .map_err(|_| crate::Error::InternalError)?;
    let credential_id = passkey.cred_id().as_slice().to_vec();
    let passkey_json = serde_json::to_value(passkey).map_err(|_| crate::Error::InternalError)?;

    insert_passkey(user_id, credential_id, passkey_json, label).await?;
    consume_challenge(challenge.id).await?;

    Ok(())
}

#[server]
pub async fn start_passkey_login(phone: String) -> Result<PasskeyChallenge> {
    use crate::models::passkey::{insert_challenge, list_user_passkeys};
    use crate::models::user::get_user_by_phone;
    use crate::service::passkeys::get_webauthn;
    use webauthn_rs::prelude::*;

    let user = get_user_by_phone(&phone).await?;
    let user_id = Uuid::parse_str(&user.id).map_err(|_| crate::Error::InternalError)?;
    let passkeys = list_user_passkeys(user_id).await?;

    if passkeys.is_empty() {
        return Err(crate::Error::NotFound);
    }

    let passkeys = passkeys
        .into_iter()
        .map(|record| serde_json::from_value::<Passkey>(record.passkey))
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|_| crate::Error::InternalError)?;
    let webauthn = get_webauthn().map_err(|_| crate::Error::InternalError)?;
    let (public_key, state) = webauthn
        .start_passkey_authentication(&passkeys)
        .map_err(|_| crate::Error::InternalError)?;
    let state = serde_json::to_value(state).map_err(|_| crate::Error::InternalError)?;
    let public_key = serde_json::to_value(public_key).map_err(|_| crate::Error::InternalError)?;
    let challenge_id = insert_challenge(user_id, "authentication", state).await?;

    Ok(PasskeyChallenge {
        challenge_id,
        public_key,
    })
}

#[server]
pub async fn finish_passkey_login(challenge_id: Uuid, credential: Value) -> Result<()> {
    use crate::models::passkey::{
        consume_challenge, load_challenge, load_passkey_by_credential_id,
        update_passkey_after_authentication,
    };
    use crate::service::passkeys::get_webauthn;
    use axum_session::SessionAnySession;
    use webauthn_rs::prelude::*;

    let challenge = load_challenge(challenge_id, "authentication").await?;
    let state: PasskeyAuthentication =
        serde_json::from_value(challenge.state).map_err(|_| crate::Error::InternalError)?;
    let credential: PublicKeyCredential =
        serde_json::from_value(credential).map_err(|_| crate::Error::InternalError)?;
    let webauthn = get_webauthn().map_err(|_| crate::Error::InternalError)?;
    let auth_result = webauthn
        .finish_passkey_authentication(&credential, &state)
        .map_err(|_| crate::Error::Unauthorized)?;

    let credential_id = auth_result.cred_id().as_slice();
    let record = load_passkey_by_credential_id(credential_id).await?;
    let mut passkey: Passkey =
        serde_json::from_value(record.passkey).map_err(|_| crate::Error::InternalError)?;
    let _ = passkey.update_credential(&auth_result);
    let passkey = serde_json::to_value(passkey).map_err(|_| crate::Error::InternalError)?;

    update_passkey_after_authentication(credential_id, passkey).await?;
    consume_challenge(challenge.id).await?;

    let Some(session) = use_context::<SessionAnySession>() else {
        return Err(crate::Error::Unauthorized);
    };
    session.set_longterm(true);
    session.set("id", challenge.user_id.to_string());
    leptos_axum::redirect("/app");

    Ok(())
}
