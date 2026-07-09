use crate::components::PasskeyRegistrationButton;
use leptos::prelude::*;

#[component]
pub fn PasskeySetup() -> impl IntoView {
    view! {
        <section class="stack">
            <div class="card wide">
                <h2>"Set up your passkey"</h2>
                <p>
                    "Create a passkey to finish signing in. After this, use your passkey for normal login."
                </p>
                <PasskeyRegistrationButton/>
            </div>
        </section>
    }
}
