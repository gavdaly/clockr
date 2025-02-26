use crate::components::loading_progress::Loading;
use crate::components::menu::Menu;
use crate::functions::user::get_current_user;
use crate::models::user::CurrentUser;
use crate::screens::{
    Auth, Dashboard, HomePage, MagicLink, //PhoneNumber,
    // TimeSheetDisplay, TimeSheetEdit, TimeSheetsAdjustment, TimeSheetsList, TimeSheetsPending,
    UserCreate, UserUpdate,
    // Users, UsersList,
};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, FlatRoutes};
use leptos_router::*;
use reactive_stores::Store;
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
fn UserWrapper() -> impl IntoView {
    let user_resource = Resource::new(
        || (),
        async move |_| {
            match get_current_user().await {
                Ok(u) => u,
                Err(e) => {
                    tracing::error!("Failed to get current user: {}", e);
                    CurrentUser::Guest
                }
            }
        }
    );
    view! {
        <Suspense fallback=Loading>
            {move || match user_resource.read().clone().take() {
                Some(user) => {
                    let user = Store::new(user);
                    view! { <ATwo user/> }.into_any()
                }
                _ => view! {}.into_any(),
            }}

        </Suspense>
    }
}

#[component]
fn ATwo(user: Store<CurrentUser>) -> impl IntoView {
    view! { <pre>{move || { format!("{:#?}", user.get()) }}</pre> }
}

#[component]
#[tracing::instrument]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let user_resource = Resource::new(
        || (),
        async move |_| {
            match get_current_user().await {
                Ok(u) => u,
                Err(e) => {
                    tracing::error!("Failed to get current user: {}", e);
                    CurrentUser::Guest
                }
            }
        }
    );

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
                <span class="version">{VERSION.clone()}</span>
            </h1>
        </header>
        <Router>
            <Suspense fallback=move || {
                view! { <Loading/> }
            }>
                {move || match user_resource.read().clone() {
                    Some(user) => view! { <Menu user/> }.into_any(),
                    _ => view! {}.into_any(),
                }}

            </Suspense>

            <UserWrapper/>
            <main id="main">

                <Suspense fallback=Loading>
                    {move || match user_resource.read().clone() {
                        Some(user) => {
                            view! { <pre>{move || { format!("{:#?}", user) }}</pre> }.into_any()
                        }
                        _ => view! {}.into_any(),
                    }}

                </Suspense>

                <FlatRoutes fallback=Loading>
                    <Route path=path!("") view=HomePage/>
                    <Route path=path!("/p/:phone") view=Auth/>
                    <Route path=path!("/login") view=PhoneNumber/>
                    <Route path=path!("/l/:link") view=MagicLink/>
                    <Route path=path!("/app") view=move || view! { <Dashboard/> }/>

                    // <Route path=path!("/app/timesheet") view=TimeSheetDisplay/>
                    // <Route path=path!("/app/admin/timesheets") view=TimeSheetsList/>
                    // <Route path=path!("/app/admin/timesheets/pending") view=TimeSheetsPending/>
                    // <Route path=path!("/app/admin/users") view=UsersList/>
                    <Route path=path!("/app/admin/users/create") view=UserCreate/>
                    <Route path=path!("/app/admin/users/edit/:id") view=UserUpdate/>
                </FlatRoutes>
            </main>
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
#[tracing::instrument]
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
