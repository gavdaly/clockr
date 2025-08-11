use crate::{Result,components::icon::Icon};
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
                        view! {
                            <ActionForm action=authenticate>
                                <div class="stack">
                                    <input type="hidden" value=query.phone name="phone"/>
                                    <label id="pin">"Enter Pin From SMS"</label>
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
    use crate::models::user::get_user_by_phone;
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

