use crate::components::loading_progress::Loading;
use crate::components::menu::Menu;
use crate::functions::user::{use_user, UserProvider};
use crate::models::user::CurrentUser;
use crate::screens::{
    Auth, Dashboard, HomePage, MagicLink, TimeSheetDisplay, TimeSheetEdit, TimeSheetMissing,
    TimeSheets, TimeSheetsAdjustment, TimeSheetsList, TimeSheetsPending, UserCreate, UserUpdate,
    Users, UsersList,
};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::nested_router::Outlet;
use leptos_router::*;
use serde::{Deserialize, Serialize};

pub static VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
#[tracing::instrument]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let content = r#"oklch(36.94% 0.1685 354.12)"#;

    tracing::info!("App component Rendered");

    view! {
        <Router>
            <UserProvider>
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
                        <span class="version">{VERSION.clone()}</span>
                    </h1>
                </header>

                {move || {
                    let user_context = use_user();
                    match user_context.get() {
                        CurrentUser::Authenticated(user) => view! { <Menu user /> }.into_any(),
                        CurrentUser::Guest => view! { <span>"log in"</span> }.into_any()
                    }
                }}

                <main id="main">
                    <Routes fallback=Loading>
                        <Route path=path!("") view=HomePage/>
                        <Route path=path!("/p/:phone") view=Auth/>
                        <Route path=path!("/login") view=PhoneNumber/>
                        <Route path=path!("/l/:link") view=MagicLink/>
                        <ParentRoute path=path!("/app") view=Outlet>
                            <Route path=path!("") view=Dashboard/>
                            <Route path=path!("/timesheet") view=TimeSheetDisplay/>
                            <Route path=path!("/timesheet/edit/:uuid") view=TimeSheetEdit/>
                            <Route path=path!("/timesheet/missing") view=TimeSheetMissing/>
                            <Route path=path!("/users") view=Users/>
                            <ParentRoute path=path!("/admin") view=move || view! { <Outlet/> }>
                                <ParentRoute path=path!("/timesheets") view=TimeSheets>
                                    <Route path=path!("") view=TimeSheetsList/>
                                    <Route path=path!("/adjustment") view=TimeSheetsAdjustment/>
                                    <Route path=path!("/pending") view=TimeSheetsPending/>
                                </ParentRoute>
                                <Route path=path!("/users") view=move || view! { <UsersList/> }/>
                                <Route
                                    path=path!("/users/create")
                                    view=move || view! { <UserCreate/> }
                                />
                                <Route
                                    path=path!("/users/edit/:id")
                                    view=move || view! { <UserUpdate/> }
                                />
                            </ParentRoute>
                        </ParentRoute>
                    </Routes>
                </main>
            </UserProvider>
        </Router>
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Status {
    user_name: String,
    checked_in_time: Option<u64>,
    user_type: String,
}

#[server]
async fn get_session_status() -> Result<bool, ServerFnError> {
    use crate::models::sessions::get_open_sessions;
    use axum_session::SessionAnySession;
    use leptos::prelude::server_fn::error::*;
    use uuid::Uuid;

    let session = use_context::<SessionAnySession>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Session missing.".into()))?;
    let id = session.get::<Uuid>("id").ok_or_else(|| {
        ServerFnError::<NoCustomError>::ServerError("Error getting Session!".into())
    })?;

    let Ok(result) = get_open_sessions(&id).await else {
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "Error getting one".into(),
        ));
    };
    Ok(!result.is_empty())
}

#[server]
async fn submit_phone_number(phone: String) -> Result<(), ServerFnError> {
    use crate::models::pins::Pin;
    use crate::models::user::get_user_by_phone;
    use crate::service::sms::send_message;
    use leptos::prelude::server_fn::error::*;

    let phone = crate::utils::filter_phone_number(&phone);

    tracing::info!("phone: {:?}", phone);

    let Ok(user) = get_user_by_phone(&phone).await else {
        tracing::error!("Could not find phone number: {:?}", phone);
        return Err(ServerFnError::Deserialization(
            "Could not Find Phone Number!".into(),
        ));
    };

    tracing::info!("user: {:?}", user);

    let Ok(pin) = Pin::create_pin_for(user.id).await else {
        tracing::error!("Could not create pin: {}", user.id.to_string());
        return Err(ServerFnError::ServerError("Error Creating Pin!".into()));
    };

    let pin_number = pin.number.to_string();
    let message = format!("Your verification code is: {pin_number}. Do not share it.");
    send_message(message, format!("+1{phone}")).await;

    leptos_axum::redirect(&("/p/".to_string() + &phone));

    Ok(())
}

#[component]
pub fn PhoneNumber() -> impl IntoView {
    let submit = ServerAction::<SubmitPhoneNumber>::new();
    let value = submit.value();
    view! {
        <Title text="Dental Care | Authentication"/>

        <ActionForm action=submit>
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
            <button type="submit">"Get Pin"</button>
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
