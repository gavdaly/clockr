use crate::components::app_context::{create_app_context, AppContext};
use crate::components::check_in::CheckInView;
use crate::components::loading_progress::Loading;
use crate::components::menu::Menu;
use crate::screens::authenticate::Auth;
use crate::screens::clock_in_link::ClockInLink;
use crate::screens::home::HomePage;
use crate::screens::magic_link::MagicLink;
use crate::screens::timesheet::{TimeSheetDisplay, TimeSheetEdit, TimeSheetMissing};
use crate::screens::timesheets::{
    TimeSheets, TimeSheetsAdjustment, TimeSheetsList, TimeSheetsPending,
};
use crate::screens::users::{AdminUsers, UserCreate, UserUpdate, Users, UsersList};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{ParentRoute, ProtectedParentRoute, Route, Router, Routes};
use leptos_router::nested_router::Outlet;
use leptos_router::path;
use serde::{Deserialize, Serialize};

pub static VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

// <Link rel="icon" type_="image/png" sizes="48x48" href="/logo-48.png"/>
// <Link rel="icon" type_="image/svg+xml" sizes="any" href="/logo.svg"/>
// <Link rel="apple-touch-icon" href="/apple-touch-icon.png"/>
// <link rel="manifest" href="/site.webmanifest" />

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
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
pub fn App() -> impl IntoView {
    provide_meta_context();

    let context = create_app_context();
    provide_context(context.clone());

    let app_context = use_context::<AppContext>().expect("should be provided");

    let content = r#"oklch(36.94% 0.1685 354.12)"#;

    view! {

        <Title text="Clkr"/>
        <Meta name="theme-color" content />

        <Stylesheet id="leptos" href="/pkg/clkr.css"/>
        <Router>
            <header id="header">
                <h1>
                    <span>"Click "</span>
                    <span class="version">{VERSION.clone()}</span>
                </h1>
            </header>

            <Menu />

            <main id="main">
                <Routes fallback=Loading>
                    <Route path=path!("/p/:phone") view=Auth />
                    <Route path=path!("/l/:link") view=MagicLink/>
                    <ProtectedParentRoute
                        path=path!("")
                        fallback=Loading
                        condition=move || Some(app_context.user.is_some())
                        view=Outlet
                        redirect_path=||"/login"
                        >

                        <Route path=path!("") view=HomePage/>
                        <Route
                            path=path!("/c/:link")
                            view=ClockInLink
                        />
                        <ParentRoute path=path!("/app") view= || view! { <Outlet/> }>
                        <Route path=path!("") view=HomePage/>
                            <Route path=path!("/timesheet") view=TimeSheetDisplay/>
                            <Route path=path!("/timesheet/edit/:uuid") view=TimeSheetEdit/>
                            <Route path=path!("/timesheet/missing") view=TimeSheetMissing/>
                            <Route path=path!("/users") view=Users/>
                            <Route path=path!("/check_in") view=CheckInView/>
                        </ParentRoute>
                        <ParentRoute path=path!("/admin") view=move || view! { <Outlet/> }>
                            <ParentRoute path=path!("/timesheets") view=TimeSheets>
                                <Route path=path!("") view=TimeSheetsList/>
                                <Route path=path!("/adjustment") view=TimeSheetsAdjustment/>
                                <Route path=path!("/pending") view=TimeSheetsPending/>
                            </ParentRoute>
                            <ParentRoute path=path!("/users") view=AdminUsers>
                                <Route path=path!("") view=UsersList/>
                                <Route path=path!("/create") view=UserCreate/>
                                <Route path=path!("/edit/:id") view=UserUpdate/>
                            </ParentRoute>
                        </ParentRoute>
                    </ProtectedParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

#[island]
fn Counter() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;
    view! {
        <button on:click=on_click>"Click Me: " {count}</button>
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
    match get_open_sessions(&id).await {
        Ok(a) => Ok(!a.is_empty()),
        Err(_) => Err(ServerFnError::<NoCustomError>::ServerError(
            "Error getting one".into(),
        )),
    }
}

#[cfg(feature = "ssr")]
async fn _is_close(latitude: f64, longitude: f64, accuracy: f64) -> Result<(), ServerFnError> {
    use crate::models::location_trackers::insert;
    use crate::utils::caluclate_distance;
    use std::env;

    let base_latitude: f64 = env::var("LATITUDE")
        .expect("To have ENV VAR: LATITUDE")
        .parse::<f64>()
        .expect("`LATITUDE` to be a floating point number");
    let base_longitude: f64 = env::var("LONGITUDE")
        .expect("To have ENV VAR: LONGITUDE")
        .parse::<f64>()
        .expect("`LONGITUDE` to be a floating point number");
    let base_accuracy: f64 = env::var("ACCURACY")
        .expect("To have ENV VAR: ACCURACY")
        .parse::<f64>()
        .expect("`ACCURACY` to be a floating point number");

    let _ = insert(latitude, longitude, accuracy).await.map_err(|_e|
            // leptos::tracing::error!("Insert Tracing Error: {:?}", e)
            ());
    if caluclate_distance(latitude, longitude, base_latitude, base_longitude) > base_accuracy {
        return Err(ServerFnError::Request("You are too far away.".into()));
    };
    if accuracy > base_accuracy {
        return Err(ServerFnError::Request(
            "The location is not accurate enough.".into(),
        ));
    };
    Ok(())
}

#[server]
async fn submit_phone_number(phone: String) -> Result<(), ServerFnError> {
    use crate::models::pins::Pin;
    use crate::models::user::get_user_by_phone;
    use crate::service::sms::send_message;
    use leptos::prelude::server_fn::error::*;

    let phone = crate::utils::filter_phone_number(&phone);

    // leptos::tracing::info!("**| phone: {:?}", phone);

    let Ok(user) = get_user_by_phone(&phone).await else {
        // leptos::tracing::warn!("Could not find phone number: {:?}", phone);
        return Err(ServerFnError::Deserialization(
            "Could not Find Phone Number!".into(),
        ));
    };

    // leptos::tracing::info!("**| user: {:?}", user);

    let Ok(pin) = Pin::create_pin_for(user.id).await else {
        // leptos::tracing::error!("Could not create pin: {}", user.id.to_string());
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
                Some(Err(e)) => view! { <div data-state="error">"Error: " {e.to_string()}</div> }.into_any(),
                _ => view! { <div data-state="error">"something is messed up"</div> }.into_any(),
            }}

        </Show>
    }
}
