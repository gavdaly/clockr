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

    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    dbg!(ulid::Ulid::new().to_string());

    if let Err(e) = jobs().await {
        error!("jobs failed: {}", e);
    }

    if let Err(e) = clkr::database::init_db().await {
        error!("database failed to initialize.");
        panic!("database failed: {:#?}", e);
    }

    let session_config = SessionConfig::default().with_table_name("user_sessions");

    let database_url = std::env::var("DATABASE_URL").expect("no database url specify");

    let poll = SessionAnyPool::new(SessionPgPool::from(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url.as_str())
            .await
            .expect("should connect to the db"),
    ));

    let session_store = SessionStore::<SessionAnyPool>::new(Some(poll), session_config)
        .await
        .expect("session store could not be created");

    let conf = get_configuration(None).unwrap();
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

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("should bind to address");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
