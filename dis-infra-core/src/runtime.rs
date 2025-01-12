use crate::core::{NodeConstructor, UntypedNode};
use crate::error::InfraError;
use futures::stream::FuturesUnordered;
use std::fs::read_to_string;
use std::path::Path;
use tokio::runtime::Runtime;
use tokio::signal;
use tracing::trace;

const COMMAND_CHANNEL_CAPACITY: usize = 50;
const EVENT_CHANNEL_CAPACITY: usize = 50;

/// The `InfraRuntime` is used to construct a specific infrastructure using composable Nodes, connected through Channels.
/// `InfraRuntime` wraps a tokio async Runtime, and manages the generic communication channels for the infrastructure.
pub struct InfraRuntime {
    command_tx: tokio::sync::broadcast::Sender<Command>,
    event_tx: tokio::sync::broadcast::Sender<Event>,
    node_factories: Vec<(&'static str, NodeConstructor)>,
}

impl InfraRuntime {
    /// Initialise a `InfraRuntime` environment.
    /// The needed communication channels are created using this function.
    pub fn init() -> Self {
        let (command_tx, _command_rx) = tokio::sync::broadcast::channel(COMMAND_CHANNEL_CAPACITY);
        let (event_tx, _event_rx) = tokio::sync::broadcast::channel(EVENT_CHANNEL_CAPACITY);

        let node_factory = crate::core::builtin_nodes();

        Self {
            command_tx,
            event_tx,
            node_factories: node_factory,
        }
    }

    /// Executes an infra specification from a given `Path`
    pub async fn run_from_path(&self, path: &Path) -> Result<(), InfraError> {
        let contents = read_to_string(path).map_err(|err| InfraError::InvalidSpec {
            message: err.to_string(),
        })?;
        self.run_infra(&contents).await
    }

    /// Executes an infra specification, provided as the bare content (TOML)
    pub async fn run_from_str(&self, toml_spec: &str) -> Result<(), InfraError> {
        self.run_infra(toml_spec).await
    }

    /// Constructs and executes the provided specification.
    async fn run_infra(&self, spec: &str) -> Result<(), InfraError> {
        let contents: toml::Table =
            toml::from_str(spec).map_err(|err| InfraError::InvalidSpec {
                message: err.to_string(),
            })?;
        // TODO (1. Read the meta info of the config, if any)

        // 2a. Get a list of all the nodes
        // 2b. Construct all nodes as a Vec<Box<dyn NodeData>>, giving them a unique id (index of the vec).
        let mut nodes: Vec<UntypedNode> = crate::core::construct_nodes_from_spec(
            &self.node_factories,
            self.command_tx.clone(),
            self.event_tx.clone(),
            &contents,
        )?;

        // 3. Construct all edges between the nodes from the spec.
        crate::core::register_channels_for_nodes(&contents, &mut nodes)?;

        // 4. Spawn the coordination tasks: shutdown signal and error event listeners
        let shutdown_handle = tokio::spawn(shutdown_signal(self.command_tx.clone()));
        let event_listener_handle = tokio::spawn(event_listener(
            self.event_tx.subscribe(),
            self.command_tx.clone(),
        ));

        // 5. Spawn all nodes by iterating the vec, collecting JoinHandles in a FuturesUnordered, or an error when a node failed to start
        let handles: Result<FuturesUnordered<_>, _> = nodes
            .into_iter()
            .map(|node| node.spawn_into_runner())
            .collect();
        let handles = handles?; // Propagate any error

        // 6. Push coordination handles to the FuturesUnordered
        handles.push(shutdown_handle);
        handles.push(event_listener_handle);

        // 7. Wait for all tasks to finish
        let _ = futures::future::join_all(handles).await;

        Ok(())
    }

    /// Obtain a sender handle to the command channel.
    pub fn command_channel(&self) -> tokio::sync::broadcast::Sender<Command> {
        self.command_tx.clone()
    }

    /// Obtain a receiver handle to the event channel.
    pub fn event_channel(&self) -> tokio::sync::broadcast::Receiver<Event> {
        self.event_tx.subscribe()
    }
}

/// General task that listens to emitted `Event`s.
///
/// This task is responsible for outputting any errors, and cleaning up the Runtime in such a case.
async fn event_listener(
    mut event_rx: tokio::sync::broadcast::Receiver<Event>,
    command_tx: tokio::sync::broadcast::Sender<Command>,
) {
    let mut command_rx = command_tx.subscribe();
    loop {
        tokio::select! {
            Ok(event) = event_rx.recv() => {
                match event {
                    Event::NodeError(err) => {
                        println!("{err}");
                        let _ = command_tx.send(Command::Quit);
                    }
                    Event::SendStatistics => {
                        // TODO
                    }
                }
            }
            command = command_rx.recv() => {
                match command {
                    Ok(command) => {
                        if command == Command::Quit {
                            break;
                        }
                    }
                    Err(err) => {
                        println!("{err}");
                        break;
                    }
                }
            }
        }
    }
}

/// General task that is spawned to listen for `Ctrl+C` and shutdown signals from the OS.
/// When a shutdown signal is detected, a `Command::Quit` command will be issued to all tasks listening for `Command`s.
async fn shutdown_signal(cmd: tokio::sync::broadcast::Sender<Command>) {
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

    trace!("Shutdown signal detected, stopping the infrastructure.");
    let _ = cmd.send(Command::Quit);
}

/// Creates a default tokio runtime.
///
/// The runtime is multithreaded, enables IO and Time features, and enters the runtime context.
pub fn default_tokio_runtime() -> Result<Runtime, InfraError> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .map_err(|_err| InfraError::CannotStartRuntime)?;
    let _guard = runtime.enter();

    Ok(runtime)
}

/// Enum of Commands that the Runtime can send to Nodes
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Quit,
}

/// Enum of Events that the Runtime and Nodes can emit
#[derive(Clone)]
pub enum Event {
    NodeError(InfraError),
    SendStatistics,
}
