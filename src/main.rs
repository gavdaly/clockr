#[cfg(feature = "ssr")]
mod jobs;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use axum_session::{SessionAnyPool, SessionConfig, SessionLayer, SessionStore};
    use axum_session_sqlx::SessionPgPool;
    use dotenv;
    use jobs::jobs;
    use leptos::config::get_configuration;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::postgres::PgPoolOptions;
    use staff::app::{shell, App};

    jobs().await.expect("jobs should run");

    // simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    dotenv::dotenv().ok();

    staff::database::init_db()
        .await
        .expect("Should create database pool");

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
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(SessionLayer::new(session_store))
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .expect("axum should serve the application");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
