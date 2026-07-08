use leptos::prelude::*;

#[component]
pub fn PasskeyRegistrationButton() -> impl IntoView {
    let (pending, set_pending) = signal(false);
    let (message, set_message) = signal(None::<String>);

    let register = move |_| {
        set_pending.set(true);
        set_message.set(None);

        #[cfg(feature = "hydrate")]
        wasm_bindgen_futures::spawn_local(async move {
            let result: std::result::Result<(), String> = async {
                let challenge = crate::functions::start_passkey_registration()
                    .await
                    .map_err(|error| error.to_string())?;
                let credential =
                    crate::service::passkeys::create_passkey_credential(challenge.public_key)
                        .await
                        .map_err(|error| error.to_string())?;
                crate::functions::finish_passkey_registration(
                    challenge.challenge_id,
                    credential,
                    Some("Default passkey".to_string()),
                )
                .await
                .map_err(|error| error.to_string())
            }
            .await;

            match result {
                Ok(()) => set_message.set(Some("Passkey added.".to_string())),
                Err(error) => set_message.set(Some(error.to_string())),
            }
            set_pending.set(false);
        });

        #[cfg(not(feature = "hydrate"))]
        {
            set_message.set(Some(
                "Passkey registration requires browser support.".to_string(),
            ));
            set_pending.set(false);
        }
    };

    view! {
        <button type="button" disabled=move || pending.get() on:click=register>
            {move || if pending.get() { "Adding passkey" } else { "Add passkey" }}
        </button>
        <Show when=move || message.get().is_some()>
            <p>{move || message.get().unwrap_or_default()}</p>
        </Show>
    }
}
