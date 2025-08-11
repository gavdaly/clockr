use crate::components::{icon::Icon, loading_progress::Loading};
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

#[derive(Clone, Params, PartialEq)]
struct PhoneParams {
    phone: String,
}

#[component]
pub fn Auth(authenticate: ServerAction<Authenticate>) -> impl IntoView {
    let (pin_input, set_pin_input) = signal(String::with_capacity(6));

    let phone_params = use_params::<PhoneParams>();
    let pattern = "[0-9]{6}";

    let value = authenticate.value();

    Effect::new(move |_| {
        if pin_input().len() == 6 {
            leptos::logging::log!("Reached Max Length")
        }
    });

    view! {
        <section class="center-center">

            <Show
                when=move || phone_params().is_ok()
                fallback=move || {
                    view! { <div>"Should not see"</div> }
                }
            >

                {move || match phone_params() {
                    Ok(query) => {
                        view! {
                            <ActionForm action=authenticate>
                                <input type="hidden" value=query.phone name="phone"/>
                                <label id="pin">"Enter Pin From SMS"</label>
                                <input
                                    type="number"
                                    name="pin"
                                    pattern=pattern
                                    inputmode="numeric"
                                    on:input=move |v| set_pin_input(event_target_value(&v))
                                />
                                <button type="submit">
                                    <Icon name="login".into()/>
                                    <span>"Log In"</span>
                                </button>

                            </ActionForm>
                            <Show when=authenticate.pending()>
                                <div>
                                    <Loading/>
                                </div>
                            </Show>
                            <Show when=move || value.with(Option::is_some)>
                                <div>{value}</div>
                            </Show>
                        }
                            .into_any()
                    }
                    Err(_e) => {
                        view! {
                            <div>
                                <input type="hidden" value="" name="phone"/>
                                <input type="hidden" name="pin"/>
                                <Show when=move || value.with(Option::is_some)>
                                    <div>{value}</div>
                                </Show>
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
async fn authenticate(pin: i32, phone: String) -> Result<(), ServerFnError> {
    use crate::app::server_fn::error::NoCustomError;
    use crate::models::pins::Pin;
    use crate::models::user::get_user_by_phone;
    use axum_session::SessionAnySession;

    let Ok(pin) = Pin::get_pin(pin).await else {
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "Internal Server Error".into(),
        ));
    };

    let Ok(user) = get_user_by_phone(&phone).await else {
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "Internal Server Error".into(),
        ));
    };

    let session = use_context::<SessionAnySession>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Session missing.".into()))?;

    if pin.user_id != user.id {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Unauthorized Try Again!".into(),
        ));
    }
    session.set_longterm(true);
    session.set("id", user.id);
    leptos_axum::redirect("/app");
    Ok(())
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use crate::app::server_fn::error::NoCustomError;
    use axum_session::SessionAnySession;
    let session = use_context::<SessionAnySession>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Session missing.".into()))?;
    session.clear();

    leptos_axum::redirect("/");
    Ok(())
}
