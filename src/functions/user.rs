use crate::models::user::{UserToday, CurrentUser};
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use tracing::{error, info, trace};

/// Retrieves the current user from the session, if one exists
///
/// # Returns
/// - `Ok(Some(UserToday))` if a user is found and authenticated
/// - `Ok(None)` if no user is authenticated
/// - `Err(ServerFnError)` if there are session or database errors
#[server]
#[tracing::instrument]
pub async fn get_current_user() -> Result<CurrentUser, ServerFnError> {
    use axum::extract::Extension;
    use axum_session::Session;
    use axum_session_sqlx::SessionPgPool;
    use leptos::prelude::server_fn::error::*;
    use leptos_axum::extract;
    use crate::models::UserDB;

    let session: Extension<Session<SessionPgPool>> = extract().await?;
    
    info!("Session: {:?}", session);

    let Some(id) = session.get("id") else {
        trace!("User not Authenticated, Could not find ID: {:?}", session);
        session.clear();
        return Ok(CurrentUser::Guest);
    };

    let Ok(user) = UserDB::get(id).await else {
        error!("Could not find User for session");
        session.clear();
        return Ok(CurrentUser::Guest);
    };


    Ok(CurrentUser::Authenticated(user.into()))
}

/// Retrieves and verifies the currently authenticated user
///
/// # Returns
/// - `Ok(UserToday)` if a valid user is found
/// - `Err(ServerFnError)` if session is missing, user is not authenticated, or user cannot be found
#[server]
#[tracing::instrument]
async fn check_in() -> Result<(), ServerFnError> {
    use crate::models::TimeLogDB;
    use axum::extract::Extension;
    use axum_session::Session;
    use leptos::prelude::server_fn::error::*;
    use axum_session_sqlx::SessionPgPool;
    use leptos_axum::extract;
    use uuid::Uuid;

    let session: Extension<Session<SessionPgPool>> = extract().await?;
    info!("Session: {:?}", session);

    let Some(id) = session.get::<String>("id") else {
        trace!("User not Authenticated, Could not find ID: {:?}", session);
        return Err(ServerFnError::<NoCustomError>::ServerError("User not authenticated".into()));
    };

    let id = Uuid::parse_str(&id).expect("Should be valid uuid");

    TimeLogDB::add(id).await?;

    Ok(())
}

/// Creates and provides the user context to all children
#[component]
pub fn UserProvider(children: Children) -> impl IntoView {
    // Create a signal to store the current user state
    let user = RwSignal::new(CurrentUser::default());
    
    // Create the resource to load the user
    let _user_resource = Resource::new(
        || (),
        move |_| async move {
            match get_current_user().await {
                Ok(u) => {
                    user.set(u);
                },
                Err(e) => {
                    tracing::error!("Failed to get current user: {}", e);
                    user.set(CurrentUser::Guest);
                }
            }
        }
    );

    // Provide the user context directly
    provide_context(user);

    view! { <Suspense fallback=move || view! { <span>"Loading..."</span> }>{children()}</Suspense> }
}

/// Helper function to get the current user signal
pub fn use_user() -> RwSignal<CurrentUser> {
    use_context::<RwSignal<CurrentUser>>().expect("User context not found. Did you wrap your app in <UserProvider>?")
}

