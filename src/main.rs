#[cfg(feature = "ssr")]
mod app_state;

#[cfg(feature = "ssr")]
mod jobs;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use app_state::AppState;
    use axum::{
        body::Body as AxumBody,
        extract::{Path, State},
        http::Request,
        response::{IntoResponse, Response},
        routing::get,
        Router,
    };
    use axum_session::{SessionAnyPool, SessionAnySession, SessionConfig, SessionLayer, SessionStore};
    use axum_session_sqlx::SessionPgPool;
    use sqlx::postgres::PgPoolOptions;
    use dotenv;
    use jobs::jobs;
    use leptos::{get_configuration, provide_context};
    use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
    use staff::app::App;
    use staff::fileserv::file_and_error_handler;

    jobs().await.expect("jobs should run");

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

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
                .unwrap(),
        ));


    let session_store = SessionStore::<SessionAnyPool>::new(Some(poll), session_config)
    .await
        .expect("session store could not be created");
    
    async fn server_fn_handler(
        session_store: SessionAnySession,
        State(_app_state): State<AppState>,
        path: Path<String>,
        request: Request<AxumBody>,
    ) -> impl IntoResponse {
        log::info!("{:?}", path);

        handle_server_fns_with_context(
            move || {
                provide_context(session_store.clone());
            },
            request,
        )
        .await
    }

    async fn leptos_routes_handler(
        session_store: SessionAnySession,
        State(app_state): State<AppState>,
        req: Request<AxumBody>,
    ) -> Response {
        let handler = leptos_axum::render_route_with_context(
            app_state.leptos_options.clone(),
            app_state.routes.clone(),
            move || {
                provide_context(session_store.clone());
            },
            App,
        );
        handler(req).await.into_response()
    }

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options,
        routes: routes.clone(),
    };

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .layer(SessionLayer::new(session_store))
        .fallback(file_and_error_handler)
        .with_state(app_state);

    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
