#[cfg(feature = "ssr")]
mod jobs;

/// Main entry point for the server-side rendering (SSR) application.
///
/// # Features
///
/// This function is only available when the "ssr" feature is enabled.
///
/// # Process
///
/// 1. Sets up logging and tracing
/// 2. Initializes background jobs
/// 3. Sets up the database and sessions
/// 4. Configures the web server with Leptos and Axum
/// 5. Starts listening for requests
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    #[allow(unused_imports)]
    use axum::body::Body;
    #[allow(unused_imports)]
    use axum::extract::Request;
    #[allow(unused_imports)]
    use axum::response::IntoResponse;
    #[allow(unused_imports)]
    use axum::response::Response;
    use axum::Router;
    use axum_session::{SessionConfig, SessionLayer, SessionStore};
    use axum_session_sqlx::SessionPgPool;
    use clkr::app::*;
    use clkr::database::{get_db, init_db};
    use jobs::jobs;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tracing::{error, info};
    use tracing_subscriber::layer::SubscriberExt;

    dotenv::dotenv().ok();

    let subscriber = tracing_subscriber::registry().with(tracing_subscriber::fmt::layer());

    match tracing::subscriber::set_global_default(subscriber) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to set tracing subscriber: {e}"),
    }

    if let Err(e) = jobs().await {
        error!("Jobs failed: {}", e);
    }

    if let Err(e) = init_db().await {
        error!("Database failed to initialize: {:?}", e);
        std::process::exit(22);
    }

    let session_config = SessionConfig::default().with_table_name("user_sessions");

    let Ok(session_store) =
        SessionStore::<SessionPgPool>::new(Some(get_db().clone().into()), session_config).await
    else {
        error!("Session store could not be created");
        std::process::exit(23);
    };

    let conf = get_configuration(None).expect("Failed to load default configuration");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(SessionLayer::new(session_store.clone()))
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let Ok(listener) = tokio::net::TcpListener::bind(&addr).await else {
        error!("Failed to bind to address: `{addr}`");
        std::process::exit(10);
    };

    info!("listening on http://{}", &addr);
    let Ok(_) = axum::serve(listener, app.into_make_service()).await else {
        error!("Server failed to start");
        std::process::exit(11);
    };
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}

