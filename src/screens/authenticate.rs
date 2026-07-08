use crate::{components::icon::Icon, Result};
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

#[derive(Clone, Params, PartialEq)]
struct PhoneParams {
    phone: Option<String>,
}

#[component]
pub fn Auth() -> impl IntoView {
    let authenticate = ServerAction::<Authenticate>::new();
    let (pin_input, set_pin_input) = signal(String::with_capacity(6));
    let (passkey_pending, set_passkey_pending) = signal(false);
    let (passkey_error, set_passkey_error) = signal(None::<String>);

    let phone_params = use_params::<PhoneParams>();
    let pattern = "[0-9]{6}";

    Effect::new(move |_| {
        if pin_input.get().len() == 6 {
            leptos::logging::log!("Reached Max Length")
        }
    });

    view! {
        <section class="center-center">

            <Show
                when=move || phone_params.get().is_ok()
                fallback=move || {
                    view! { <div>"Should not see"</div> }
                }
            >

                {move || match phone_params.get() {
                    Ok(query) => {
                        let phone = query.phone.unwrap_or_default();
                        #[cfg(feature = "hydrate")]
                        let passkey_phone = phone.clone();
                        let login_with_passkey = move |_| {
                            set_passkey_pending.set(true);
                            set_passkey_error.set(None);

                            #[cfg(feature = "hydrate")]
                            wasm_bindgen_futures::spawn_local({
                                let phone = passkey_phone.clone();
                                async move {
                                    let result: std::result::Result<(), String> = async {
                                        let challenge =
                                            crate::functions::start_passkey_login(phone)
                                                .await
                                                .map_err(|error| error.to_string())?;
                                        let credential =
                                            crate::service::passkeys::get_passkey_credential(
                                                challenge.public_key,
                                            )
                                            .await
                                            .map_err(|error| error.to_string())?;
                                        crate::functions::finish_passkey_login(
                                            challenge.challenge_id,
                                            credential,
                                        )
                                        .await
                                        .map_err(|error| error.to_string())
                                    }
                                    .await;

                                    if let Err(error) = result {
                                        set_passkey_error.set(Some(error.to_string()));
                                    }
                                    set_passkey_pending.set(false);
                                }
                            });

                            #[cfg(not(feature = "hydrate"))]
                            {
                                set_passkey_error
                                    .set(Some("Passkey login requires browser support.".to_string()));
                                set_passkey_pending.set(false);
                            }
                        };

                        view! {
                            <div class="stack">
                                <button
                                    type="button"
                                    disabled=move || passkey_pending.get()
                                    on:click=login_with_passkey
                                >
                                    <Icon name="login"/>
                                    <span>
                                        {move || {
                                            if passkey_pending.get() {
                                                "Checking passkey"
                                            } else {
                                                "Log in with passkey"
                                            }
                                        }}
                                    </span>
                                </button>
                                <Show when=move || passkey_error.get().is_some()>
                                    <p data-state="error">
                                        {move || passkey_error.get().unwrap_or_default()}
                                    </p>
                                </Show>
                            </div>
                            <ActionForm action=authenticate>
                                <div class="stack">
                                    <input type="hidden" value=phone name="phone"/>
                                    <label id="pin">"Recovery: enter PIN from SMS"</label>
                                    <input
                                        type="number"
                                        name="pin"
                                        pattern=pattern
                                        inputmode="numeric"
                                        on:input=move |v| set_pin_input.set(event_target_value(&v))
                                    />
                                    <button type="submit">
                                        <Icon name="login"/>
                                        <span>"Log In"</span>
                                    </button>
                                </div>
                            </ActionForm>
                        }
                            .into_any()
                    }
                    Err(_e) => {
                        view! {
                            <div>
                                <input type="hidden" value="" name="phone"/>
                                <input type="hidden" name="pin"/>
                            </div>
                        }
                            .into_any()
                    }
                }}

            </Show>
        </section>
    }
}

#[server]
async fn authenticate(pin: i32, phone: String) -> Result<()> {
    use crate::models::pins::Pin;
    use crate::models::user::{get_user_by_phone, store_user_email};
    use axum_session::SessionAnySession;

    let Ok(pin) = Pin::get_pin(pin).await else {
        return Err(crate::Error::InternalError);
    };

    let Ok(user) = get_user_by_phone(&phone).await else {
        return Err(crate::Error::InternalError);
    };

    let Some(session) = use_context::<SessionAnySession>() else {
        return Err(crate::Error::Unauthorized);
    };

    if pin.user_id.to_string() != user.id {
        return Err(crate::Error::Unauthorized);
    }
    let pending_user_id = session
        .get::<String>("pending_recovery_user_id")
        .unwrap_or_default();
    let pending_email = session
        .get::<String>("pending_recovery_email")
        .unwrap_or_default();

    if pending_user_id == user.id && !pending_email.is_empty() {
        if let Err(error) = store_user_email(pin.user_id, &pending_email).await {
            tracing::warn!("Could not store recovery email: {error}");
        }
    }

    session.set("pending_recovery_user_id", String::new());
    session.set("pending_recovery_email", String::new());
    session.set_longterm(true);
    session.set("id", user.id);
    leptos_axum::redirect("/app");
    Ok(())
}

#[server]
pub async fn logout() -> Result<()> {
    use axum_session::SessionAnySession;
    let Some(session) = use_context::<SessionAnySession>() else {
        tracing::trace!("Tried to remove non existing session");
        return Err(crate::Error::InternalError);
    };
    session.clear();

    leptos_axum::redirect("/");
    Ok(())
}
