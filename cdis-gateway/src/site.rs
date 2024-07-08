use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use askama::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::response::sse::{Event as SseEvent, Sse};
use axum::Router;
use axum::routing::get;
use axum_extra::{headers, TypedHeader};
use futures::stream::Stream;
use tokio::signal;
// use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{debug, error};
use tracing::log::trace;
use crate::{Command};
use crate::config::Config;
use crate::site::templates::{CodecStatsTemplate, CodecStatsValues, ConfigMetaTemplate, ConfigTemplate, HomeTemplate, SocketStatsTemplate, SocketStatsValues, UdpEndpointValues};
use crate::stats::SseStat;

// const ASSETS_DIR: &str = "templates";

pub(crate) struct SiteState {
    config: Config,
    stats_tx: tokio::sync::broadcast::Sender<SseStat>,
    cmd_tx: tokio::sync::broadcast::Sender<Command>,
}

pub(crate) async fn run_site(config: Config,
                      stats_tx: tokio::sync::broadcast::Sender<SseStat>,
                      cmd_tx: tokio::sync::broadcast::Sender<Command>) {
    // let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(ASSETS_DIR);
    // let static_files_service = ServeDir::new(assets_dir).append_index_html_on_directories(true);

    let state = Arc::new(SiteState {
        config: config.clone(),
        stats_tx,
        cmd_tx: cmd_tx.clone(),
    });

    let router = Router::new()
        // .fallback_service(static_files_service)
        .route("/sse", get(sse_handler))
        .layer(TraceLayer::new_for_http())
        .route("/", get(home))
        .route("/styles.css", get(styles))
        .route("/index.js", get(scripts))
        .route("/script_htmx.js", get(script_htmx))
        .route("/script_htmx_sse.js", get(script_htmx_sse))
        .route("/config", get(config_info))
        .route("/meta", get(meta_info))
        .route("/clicked", get(clicked))
        .with_state(state);

    // TODO handle bind error
    let host_ip = format!("127.0.0.1:{}", config.site_host);
    let listener = tokio::net::TcpListener::bind(&host_ip)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind TCP socket for Web UI - {}", host_ip));
    tracing::debug!("Site listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal(cmd_tx.clone()))
        .await
        .unwrap();

    match cmd_tx.send(Command::Quit) {
        Ok(_) => {}
        Err(_) => { error!("Could not send Command::Quit.") }
    }
}

// TODO move to main.rs; axum shutdown signal can be listening for a Command::Quit
async fn shutdown_signal(cmd_tx: tokio::sync::broadcast::Sender<Command>) {
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

    // Send Command::Quit, resolving not stopping the axum server due to open (infinite) SSE connections.
    cmd_tx.send(Command::Quit).expect("Failed to send Command:Quit after receiving shutdown signal");
}

async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
    State(state): State<Arc<SiteState>>,
) -> Sse<impl Stream<Item = Result<SseEvent, Infallible>>> {
    debug!("`{}` connected", user_agent.as_str());

    let mut stats_rx = state.stats_tx.subscribe();
    let mut cmd_rx = state.cmd_tx.subscribe();
    let stream = async_stream::try_stream! {
        yield SseEvent::default().event("status").data("SSE connected");

        loop {
            tokio::select! {
                stat = stats_rx.recv() => {
                    if let Ok(stat) = stat {
                        yield match stat {
                            SseStat::DisSocket(stat) => {
                                SseEvent::default().event("dis_socket").data(SocketStatsTemplate {
                                    name: "DIS network",
                                    stats: SocketStatsValues::from(&stat),
                                    socket: UdpEndpointValues::from(&state.config.dis_socket),
                                }.render().unwrap())
                            }
                            SseStat::CdisSocket(stat) => {
                                SseEvent::default().event("cdis_socket").data(SocketStatsTemplate {
                                    name: "C-DIS network",
                                    stats: SocketStatsValues::from(&stat),
                                    socket: UdpEndpointValues::from(&state.config.cdis_socket),
                                }.render().unwrap())
                            }
                            SseStat::Encoder(stat) => {
                                SseEvent::default().event("encoder").data(CodecStatsTemplate {
                                    name: "Encoder",
                                    stats: CodecStatsValues::from(&stat),
                                }.render().unwrap())
                            }
                            SseStat::Decoder(stat) => {
                                SseEvent::default().event("decoder").data(CodecStatsTemplate {
                                    name: "Decoder",
                                    stats: CodecStatsValues::from(&stat),
                                }.render().unwrap())
                            }
                        }
                    }
                }
                cmd = cmd_rx.recv() => {
                    let cmd = cmd.unwrap_or_default();
                    match cmd {
                        Command::NoOp => { }
                        Command::Quit => {
                            trace!("SSE handler task stopping due to receiving Command::Quit.");
                            return;
                        }
                    }
                }
            }
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(5))
            .text("keep-alive-text"),
    )
}

pub(crate) async fn styles() -> Result<impl IntoResponse, Response> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../build/styles.css").to_owned());

    match response {
        Ok(response) => { Ok(response) }
        Err(e) => { Err((StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP error: {e}")).into_response()) }
    }
}

pub(crate) async fn scripts() -> Result<impl IntoResponse, Response> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/javascript")
        .body(include_str!("../build/index.js").to_owned());

    match response {
        Ok(response) => { Ok(response) }
        Err(e) => { Err((StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP error: {e}")).into_response()) }
    }
}

pub(crate) async fn script_htmx() -> Result<impl IntoResponse, Response> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/javascript")
        .body(include_str!("../assets/scripts/htmx-1.9.12-min.js").to_owned());

    match response {
        Ok(response) => { Ok(response) }
        Err(e) => { Err((StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP error: {e}")).into_response()) }
    }
}

pub(crate) async fn script_htmx_sse() -> Result<impl IntoResponse, Response> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/javascript")
        .body(include_str!("../assets/scripts/htmx-1.9.12-ext-sse.js").to_owned());

    match response {
        Ok(response) => { Ok(response) }
        Err(e) => { Err((StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP error: {e}")).into_response()) }
    }
}

pub(crate) async fn config_info(State(state): State<Arc<SiteState>>) -> impl IntoResponse {
    ConfigMetaTemplate {
        name: state.config.meta.name.clone(),
        author: state.config.meta.author.clone(),
        version: state.config.meta.version.clone(),
    }
}

pub(crate) async fn meta_info(State(state): State<Arc<SiteState>>) -> impl IntoResponse {
    ConfigMetaTemplate {
        name: state.config.meta.name.clone(),
        author: state.config.meta.author.clone(),
        version: state.config.meta.version.clone(),
    }
}

pub async fn clicked() -> impl IntoResponse {
    ConfigTemplate
}

pub(crate) async fn home(State(state): State<Arc<SiteState>>) -> impl IntoResponse {
    HomeTemplate {
        config: state.config.clone()
    }
}

mod templates {
    use std::collections::HashMap;
    use askama_axum::Template;
    use cdis_assemble::codec::CodecUpdateMode;
    use dis_rs::enumerations::PduType;
    use bytesize::ByteSize;
    use crate::config::{Config, UdpEndpoint, UdpMode};
    use crate::stats::{CodecStats, SocketStats};

    #[derive(Template)]
    #[template(path = "index.html")]
    pub struct HomeTemplate {
        pub(crate) config: Config,
    }

    #[derive(Template)]
    #[template(path = "config.html")]
    pub struct ConfigTemplate;

    #[derive(Template)]
    #[template(path = "config_meta.html")]
    pub struct ConfigMetaTemplate {
        pub(crate) name: String,
        pub(crate) author: String,
        pub(crate) version: String,
    }

    #[derive(Template)]
    #[template(path = "socket_stats.html")]
    pub struct SocketStatsTemplate<'a> {
        pub(crate) name: &'a str,
        pub(crate) stats: SocketStatsValues,
        pub(crate) socket: UdpEndpointValues,
    }

    pub(crate) struct SocketStatsValues {
        pub(crate) bytes_received: ByteSize,
        pub(crate) bytes_received_latest_aggregate: ByteSize,
        pub(crate) bytes_sent: ByteSize,
        pub(crate) bytes_sent_latest_aggregate: ByteSize,
        pub(crate) packets_received: u64,
        pub(crate) packets_received_latest_aggregate: u64,
        pub(crate) packets_sent: u64,
        pub(crate) packets_sent_latest_aggregate: u64,
    }

    impl From<&SocketStats> for SocketStatsValues {
        fn from(stats: &SocketStats) -> Self {
            Self {
                bytes_received: ByteSize::b(stats.bytes_received),
                bytes_received_latest_aggregate: ByteSize::b(stats.bytes_received_latest_aggregate),
                bytes_sent: ByteSize::b(stats.bytes_sent),
                bytes_sent_latest_aggregate: ByteSize::b(stats.bytes_sent_latest_aggregate),
                packets_received: stats.packets_received,
                packets_received_latest_aggregate: stats.packets_received_latest_aggregate,
                packets_sent: stats.packets_sent,
                packets_sent_latest_aggregate: stats.packets_sent_latest_aggregate,
            }
        }
    }

    pub(crate) struct UdpEndpointValues {
        pub(crate) mode: String,
        pub(crate) interface: String,
        pub(crate) address: String,
        pub(crate) ttl: u16,
        pub(crate) block_own_socket: bool,
    }

    impl From<&UdpEndpoint> for UdpEndpointValues {
        fn from(endpoint: &UdpEndpoint) -> Self {
            let mode = match endpoint.mode {
                UdpMode::UniCast => { "Unicast".to_string() }
                UdpMode::BroadCast => { "Broadcast".to_string() }
                UdpMode::MultiCast => { "Multicast".to_string() }
            };
            let interface = format!("{}:{}", endpoint.interface.ip(), endpoint.interface.port());
            let address = format!("{}:{}", endpoint.address.ip(), endpoint.address.port());

            Self {
                mode,
                interface,
                address,
                ttl: endpoint.ttl,
                block_own_socket: endpoint.block_own_socket,
            }
        }
    }

    #[derive(Template)]
    #[template(path = "codec_stats.html")]
    pub struct CodecStatsTemplate<'a> {
        pub(crate) name: &'a str,
        pub(crate) stats: CodecStatsValues,
    }

    pub(crate) struct CodecStatsValues {
        pub received_count: u64,
        pub es_count: u64,
        pub fire_count: u64,
        pub detonation_count: u64,
        pub rejected_count: u64,
        pub unimplemented_count: u64,
        pub compression_rate_total: String,
    }

    impl From<&CodecStats> for CodecStatsValues {
        fn from(stats: &CodecStats) -> Self {
            Self {
                received_count: stats.received_count.values().map(|val| val.0).sum::<u64>().saturating_sub(stats.rejected_count),
                es_count: stats.received_count.get(&PduType::EntityState).unwrap_or(&(0, 0)).0,
                fire_count: stats.received_count.get(&PduType::Fire).unwrap_or(&(0, 0)).0,
                detonation_count: stats.received_count.get(&PduType::Detonation).unwrap_or(&(0, 0)).0,
                rejected_count: stats.rejected_count,
                unimplemented_count: stats.unimplemented_count,
                compression_rate_total: if stats.compression_rate_total.is_nan() {
                    "0.0".to_string()
                } else {
                    format!("{0:.2}", stats.compression_rate_total)
                },
            }
        }
    }
}
