#[cfg(feature = "ssr")]
mod jobs;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use axum_session::{SessionAnyPool, SessionConfig, SessionLayer, SessionStore};
    use axum_session_sqlx::SessionPgPool;
    use clkr::app::*;
    use jobs::jobs;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::postgres::PgPoolOptions;
    use tracing::{error, info};
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    dotenv::dotenv().ok();

    // Create a more comprehensive subscriber with environment filtering
    let subscriber = tracing_subscriber::registry().with(tracing_subscriber::fmt::layer());

    // Set it as the global default
    match tracing::subscriber::set_global_default(subscriber) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to set tracing subscriber: {}", e),
    }

    if let Err(e) = jobs().await {
        error!("Jobs failed: {}", e);
    }

    if let Err(e) = clkr::database::init_db().await {
        error!("Database failed to initialize.");
        std::process::exit(22);
    }

    let session_config = SessionConfig::default().with_table_name("user_sessions");

    let Ok(database_url) = std::env::var("DATABASE_URL") else {
        error!("No `DATABASE_URL` environment variable specified");
        std::process::exit(21);
    };

    let poll = SessionAnyPool::new(SessionPgPool::from(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url.as_str())
            .await
            .expect("Should Connect to the Database"),
    ));

    let Ok(session_store) = SessionStore::<SessionAnyPool>::new(Some(poll), session_config).await
    else {
        error!("Session store could not be created");
        std::process::exit(23);
    };

    let conf = get_configuration(None).expect("Failed to load default configuration");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(SessionLayer::new(session_store))
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let Ok(listener) = tokio::net::TcpListener::bind(&addr).await else {
        error!("Failed to bind to address: `{addr}`");
        std::process::exit(10);
    };

    info!("listening on http://{}", &addr);
    let Ok(server) = axum::serve(listener, app.into_make_service()).await else {
        error!("Server failed to start");
        std::process::exit(11);
    };
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
