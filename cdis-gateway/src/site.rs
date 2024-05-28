use std::path::PathBuf;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response, Sse};
use axum::Router;
use axum::routing::get;
use tokio::signal;
// use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use crate::{Command, Event};
use crate::config::Config;
use crate::site::templates::{ConfigTemplate, HomeTemplate};

// const ASSETS_DIR: &str = "templates";

pub async fn run_site(config: Config,
                mut cmd_tx: tokio::sync::broadcast::Sender<Command>,
                event_rx: tokio::sync::mpsc::Receiver<Event>) {
    // let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(ASSETS_DIR);
    // let static_files_service = ServeDir::new(assets_dir).append_index_html_on_directories(true);

    let router = Router::new()
        // .fallback_service(static_files_service)
        // .route("/sse", get(sse_handler))
        .layer(TraceLayer::new_for_http())
        .route("/", get(home))
        .route("/styles.css", get(styles))
        .route("/index.js", get(scripts))
        .route("/clicked", get(clicked));

    // TODO handle bind error
    let host_ip = format!("127.0.0.1:{}", config.site_host);
    let listener = tokio::net::TcpListener::bind(&host_ip)
        .await
        .expect(format!("Failed to bind TCP socket for Web UI - {}", host_ip).as_str());
    tracing::debug!("Site listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
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

pub async fn styles() -> Result<impl IntoResponse, Response> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../build/styles.css").to_owned());

    match response {
        Ok(response) => { Ok(response) }
        Err(e) => { Err((StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP error: {e}")).into_response()) }
    }
}

pub async fn scripts() -> Result<impl IntoResponse, Response> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/javascript")
        .body(include_str!("../build/index.js").to_owned());

    match response {
        Ok(response) => { Ok(response) }
        Err(e) => { Err((StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP error: {e}")).into_response()) }
    }
}

pub async fn clicked() -> impl IntoResponse {
    ConfigTemplate
}

pub async fn home() -> impl IntoResponse {
    HomeTemplate
}

mod templates {
    use askama_axum::Template;

    #[derive(Template)]
    #[template(path = "index.html")]
    pub struct HomeTemplate;

    #[derive(Template)]
    #[template(path = "config.html")]
    pub struct ConfigTemplate;
}
