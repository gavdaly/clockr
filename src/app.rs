use crate::components::loading_progress::Loading;
use crate::components::menu::Menu;
use crate::functions::user::get_current_user;
use crate::models::CurrentUser;
use crate::screens::{
    Auth,
    Dashboard,
    HomePage,
    MagicLink, //PhoneNumber,
    PasskeySetup,
    // TimeSheetDisplay, TimeSheetEdit, TimeSheetsAdjustment, TimeSheetsList, TimeSheetsPending,
    UserCreate,
    UserUpdate,
    UsersList,
};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{FlatRoutes, Redirect, Route, Router};
use leptos_router::*;

pub static VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
fn UserProvider(children: Children) -> impl IntoView {
    let user_resource = Resource::new(
        || (),
        async move |_| match get_current_user().await {
            Ok(u) => u,
            Err(e) => {
                tracing::error!("Failed to get current user: {}", e);
                CurrentUser::Guest
            }
        },
    );
    provide_context(user_resource);
    view! { <Suspense fallback=Loading>{children()}</Suspense> }
}

#[component]
#[tracing::instrument]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let content = r#"oklch(36.94% 0.1685 354.12)"#;

    tracing::info!("App component Rendered");

    view! {
        <Title text="Clkr"/>
        <Meta name="theme-color" content=content/>

        <Stylesheet id="leptos" href="/pkg/clkr.css"/>
        <Link rel="icon" type_="image/png" sizes="48x48" href="/logo-48.png"/>
        <Link rel="icon" type_="image/svg+xml" sizes="any" href="/logo.svg"/>
        <Link rel="apple-touch-icon" href="/apple-touch-icon.png"/>
        <Link rel="manifest" href="/site.webmanifest"/>
        <header id="header">
            <h1>
                <span>"Clkr"</span>
                <span class="version">{VERSION}</span>
            </h1>
        </header>
        <Router>
            <UserProvider>
                <Menu/>

                <main id="main">

                    <FlatRoutes fallback=Loading>
                        <Route path=path!("") view=HomePage/>
                        <Route path=path!("/p/:phone") view=Auth/>
                        <Route path=path!("/login") view=PhoneNumber/>
                        <Route path=path!("/l/:link") view=MagicLink/>
                        <Route path=path!("/app/passkeys/setup") view=PasskeySetup/>
                        <Route
                            path=path!("/app")
                            view=move || {
                                view! {
                                    <PasskeyEnrollmentGate>
                                        <Dashboard/>
                                    </PasskeyEnrollmentGate>
                                }
                            }
                        />

                        // <Route path=path!("/app/timesheet") view=TimeSheetDisplay/>
                        // <Route path=path!("/app/admin/timesheets") view=TimeSheetsList/>
                        // <Route path=path!("/app/admin/timesheets/pending") view=TimeSheetsPending/>
                        <Route
                            path=path!("/app/admin/users")
                            view=move || {
                                view! {
                                    <PasskeyEnrollmentGate>
                                        <UsersList/>
                                    </PasskeyEnrollmentGate>
                                }
                            }
                        />
                        <Route
                            path=path!("/app/admin/users/create")
                            view=move || {
                                view! {
                                    <PasskeyEnrollmentGate>
                                        <UserCreate/>
                                    </PasskeyEnrollmentGate>
                                }
                            }
                        />
                        <Route
                            path=path!("/app/admin/users/edit/:id")
                            view=move || {
                                view! {
                                    <PasskeyEnrollmentGate>
                                        <UserUpdate/>
                                    </PasskeyEnrollmentGate>
                                }
                            }
                        />
                    </FlatRoutes>
                </main>
            </UserProvider>
        </Router>
    }
}

#[component]
fn PasskeyEnrollmentGate(children: ChildrenFn) -> impl IntoView {
    let setup_required = Resource::new(
        || (),
        async move |_| {
            crate::functions::passkey_setup_required()
                .await
                .unwrap_or(false)
        },
    );

    view! {
        <Suspense fallback=Loading>
            {move || match setup_required.get() {
                Some(true) => view! { <Redirect path="/app/passkeys/setup"/> }.into_any(),
                Some(false) => children().into_any(),
                None => view! { <Loading/> }.into_any(),
            }}
        </Suspense>
    }
}

#[server]
#[tracing::instrument]
async fn submit_phone_number(phone: String, email: Option<String>) -> Result<(), ServerFnError> {
    use crate::models::pins::Pin;
    use crate::models::user::get_user_by_phone;
    use crate::service::sms::send_message;
    use axum_session::SessionAnySession;
    use leptos::prelude::server_fn::error::*;

    let phone = crate::utils::filter_phone_number(&phone);
    let email = email.unwrap_or_default().trim().to_ascii_lowercase();
    let email = email.contains('@').then_some(email);

    tracing::info!("phone: {:?}", phone);
    tracing::info!("email recovery requested: {}", email.is_some());

    let Ok(user) = get_user_by_phone(&phone).await else {
        tracing::error!("Could not find phone number: {:?}", phone);
        return Err(ServerFnError::Deserialization(
            "Could not Find Phone Number!".into(),
        ));
    };

    tracing::info!("user: {:?}", user);

    let user_id = uuid::Uuid::parse_str(&user.id).expect("Should be valid uuid");
    if let Some(session) = use_context::<SessionAnySession>() {
        session.set("pending_recovery_user_id", user.id.clone());
        session.set("pending_recovery_email", email.unwrap_or_default());
    }

    let Ok(pin) = Pin::create_pin_for(user_id).await else {
        tracing::error!("Could not create pin: {}", user.id.to_string());
        return Err(ServerFnError::ServerError("Error Creating Pin!".into()));
    };

    let pin_number = pin.number.to_string();
    let message = format!("Your verification code is: {pin_number}. Do not share it.");
    send_message(message, format!("+1{phone}")).await;
    // TODO: send the same recovery challenge by email once an email service is configured.

    leptos_axum::redirect(&("/p/".to_string() + &phone));

    Ok(())
}

#[component]
pub fn PhoneNumber() -> impl IntoView {
    let submit = ServerAction::<SubmitPhoneNumber>::new();
    let value = submit.value();
    let (passkey_pending, set_passkey_pending) = signal(false);
    let (passkey_error, set_passkey_error) = signal(None::<String>);
    let login_with_passkey = move |_| {
        set_passkey_pending.set(true);
        set_passkey_error.set(None);

        #[cfg(feature = "hydrate")]
        wasm_bindgen_futures::spawn_local({
            async move {
                let result: std::result::Result<(), String> = async {
                    let challenge = crate::functions::start_passkey_primary_login()
                        .await
                        .map_err(|error| error.to_string())?;
                    let credential =
                        crate::service::passkeys::get_passkey_credential(challenge.public_key)
                            .await
                            .map_err(|error| error.to_string())?;
                    crate::functions::finish_passkey_primary_login(
                        challenge.challenge_id,
                        credential,
                    )
                    .await
                    .map_err(|error| error.to_string())
                }
                .await;

                if let Err(error) = result {
                    set_passkey_error.set(Some(error));
                }
                set_passkey_pending.set(false);
            }
        });

        #[cfg(not(feature = "hydrate"))]
        {
            set_passkey_error.set(Some("Passkey login requires browser support.".to_string()));
            set_passkey_pending.set(false);
        }
    };

    view! {
        <Title text="Dental Care | Authentication"/>

        <section class="stack">
            <button
                type="button"
                disabled=move || passkey_pending.get()
                on:click=login_with_passkey
            >
                {move || if passkey_pending.get() { "Checking passkey" } else { "Continue with passkey" }}
            </button>
            <Show when=move || passkey_error.get().is_some()>
                <div data-state="error">{move || passkey_error.get().unwrap_or_default()}</div>
            </Show>
        </section>

        <ActionForm action=submit attr:class="stack">
            <label>"Phone Number"</label>
            <input
                id="phone"
                type="tel"
                name="phone"
                autocomplete="tel"
                placeholder="+1 (893) 234-2345"
                inputmode="tel"
                required
            />
            <label>"Email"</label>
            <input
                id="email"
                type="email"
                name="email"
                autocomplete="email"
                placeholder="you@example.com"
            />
            <button type="submit">"No passkey yet: send SMS/email recovery"</button>
        </ActionForm>

        <Show when=move || {
            value.get().is_some()
        }>
            {match value.get() {
                Some(Err(e)) => {
                    view! { <div data-state="error">"Error: " {e.to_string()}</div> }.into_any()
                }
                _ => view! { <div data-state="error">"something is messed up"</div> }.into_any(),
            }}

        </Show>
    }
}
