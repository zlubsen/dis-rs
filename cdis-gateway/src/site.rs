use std::path::PathBuf;
use axum::response::Sse;
use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use crate::{Command, Event};
use crate::config::Config;

pub async fn run_site(config: Config,
                mut cmd_tx: tokio::sync::broadcast::Sender<Command>,
                event_rx: tokio::sync::mpsc::Receiver<Event>) {
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let static_files_service = ServeDir::new(assets_dir).append_index_html_on_directories(true);
    // build our application with a route
    let router = Router::new()
        .fallback_service(static_files_service)
        // .route("/sse", get(sse_handler))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    tracing::debug!("Site listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

// async fn sse_handler(
//     TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
// ) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
//     println!("`{}` connected", user_agent.as_str());
//
//     Sse::new(stream).keep_alive(
//         axum::response::sse::KeepAlive::new()
//             .interval(Duration::from_secs(1))
//             .text("keep-alive-text"),
//     )
// }
