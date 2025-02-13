#[cfg(feature = "ssr")]
pub use crate::app::{shell, App};
#[cfg(feature = "ssr")]
use axum::extract::State;
#[cfg(feature = "ssr")]
pub use axum::{
    body::Body as AxumBody,
    extract::Path,
    http::Request,
    response::{IntoResponse, Response},
};
#[cfg(feature = "ssr")]
use leptos::{config::LeptosOptions, context::provide_context};

#[cfg(feature = "ssr")]
pub async fn custom_handler(
    Path(id): Path<String>,
    State(options): State<LeptosOptions>,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        move || {
            provide_context(id.clone());
        },
        move || shell(options.clone()),
    );
    handler(req).await.into_response()
}
