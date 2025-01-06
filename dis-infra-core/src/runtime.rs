use crate::core::NodeData;
use crate::error::InfraError;
use crate::infra::{node_data_from_spec, register_channel_from_spec};
use futures::stream::FuturesUnordered;
use std::fs::read_to_string;
use std::future::Future;
use std::path::Path;
use tokio::runtime::Runtime;
use tokio::signal;
use toml::Value;

const COMMAND_CHANNEL_CAPACITY: usize = 50;
const EVENT_CHANNEL_CAPACITY: usize = 50;

/// The `InfraRuntime` is used to construct a specific infrastructure using composable Nodes, connected through Channels.
/// `InfraRuntime` wraps a tokio async Runtime, and manages the generic communication channels for the infrastructure.
pub struct InfraRuntime {
    async_runtime: Runtime,
    command_tx: tokio::sync::broadcast::Sender<Command>,
    event_tx: tokio::sync::mpsc::Sender<Event>,
    event_rx: Option<tokio::sync::mpsc::Receiver<Event>>,
}

impl InfraRuntime {
    /// Initialise a `InfraRuntime` environment, using the provided `tokio::Runtime`.
    /// The needed communication channels are created using this function.
    pub fn init(runtime: Runtime) -> Self {
        let (command_tx, _command_rx) = tokio::sync::broadcast::channel(COMMAND_CHANNEL_CAPACITY);
        let (event_tx, event_rx) = tokio::sync::mpsc::channel(EVENT_CHANNEL_CAPACITY);

        Self {
            async_runtime: runtime,
            command_tx,
            event_tx,
            event_rx: Some(event_rx),
        }
    }

    /// Construct nodes and channels based on a specification, and run the infrastructure
    pub fn run_with_spec(&mut self, path: &Path) -> Result<(), InfraError> {
        let contents = read_to_string(path).map_err(|err| InfraError::InvalidSpec {
            message: err.to_string(),
        })?;
        self.run_with_spec_string(&contents)
    }

    pub fn run_with_spec_string(&mut self, spec: &str) -> Result<(), InfraError> {
        // TODO (1. Read the meta info of the config, if any)

        // We use replace here because we cannot move event_rx out of the runtime struct normally.
        let event_rx =
            std::mem::replace(&mut self.event_rx, None).ok_or(InfraError::CannotStartRuntime)?;

        self.async_runtime.block_on(async {
            let contents: toml::Table =
                toml::from_str(spec).map_err(|err| InfraError::InvalidSpec {
                    message: err.to_string(),
                })?;

            // 2. Get a list of all the nodes
            // 3. Construct all nodes as a Vec<Box<dyn NodeData>>, giving them a unique id (index of the vec).
            let mut nodes: Vec<Box<dyn NodeData>> = if let Value::Array(array) = &contents["nodes"]
            {
                array
                    .iter()
                    .enumerate()
                    .map(|(id, node)| {
                        if let Value::Table(spec) = node {
                            match node_data_from_spec(
                                id as u64,
                                self.command_tx.subscribe(),
                                self.event_tx.clone(),
                                spec,
                            ) {
                                Ok(node) => Ok(node),
                                Err(err) => return Err(err),
                            }
                        } else {
                            return Err(InfraError::InvalidSpec {
                                message: format!("Invalid node spec for index {id}"),
                            });
                        }
                    })
                    .filter(Result::is_ok)
                    .map(|node| node.expect("We did check 'is_ok()'."))
                    .collect()
            } else {
                return Err(InfraError::InvalidSpec {
                    message:
                        "A spec file must contain a non-empty list of 'nodes', which is missing"
                            .to_string(),
                });
            };

            // 4. Get a list of all the edges (channels)
            // 5. Construct all edges by getting the nodes from the vec.
            if let Value::Array(array) = &contents["channels"] {
                for channel in array {
                    if let Value::Table(spec) = channel {
                        if let Err(err) = register_channel_from_spec(spec, &mut nodes) {
                            return Err(err);
                        }
                    }
                }
            } else {
                return Err(InfraError::InvalidSpec {
                    message:
                        "A spec file must contain a non-empty list of 'channels', which is missing"
                            .to_string(),
                });
            };

            // 6. Spawn all nodes by iterating the vec, collecting JoinHandles in a JoinSet(?)
            let handles = nodes
                .into_iter()
                .map(|node| node.spawn_into_runner())
                .collect::<FuturesUnordered<_>>();

            // Coordination: shutdown signal and error even listeners
            handles.push(tokio::spawn(shutdown_signal(self.command_tx.clone())));

            handles.push(tokio::spawn(event_listener(
                event_rx,
                self.command_tx.clone(),
            )));
            println!(
                "Spec successfully constructed. Joining on all tasks ({}).",
                handles.len()
            );
            // 7. Wait for all tasks to finish
            let _ = futures::future::join_all(handles).await;
            println!("Runtime terminated.");
            Ok(())
        })
    }
}

/// General task that listens to emitted `Event`s.
///
/// This task is responsible for outputting any errors, and cleaning up the Runtime in such a case.
async fn event_listener(
    mut event_rx: tokio::sync::mpsc::Receiver<Event>,
    command_tx: tokio::sync::broadcast::Sender<Command>,
) {
    let mut command_rx = command_tx.subscribe();
    loop {
        tokio::select! {
            Some(event) = event_rx.recv() => {
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

    println!("Shutdown signal detected, stopping");
    let _ = cmd.send(Command::Quit);
}

/// Creates a default tokio runtime.
///
/// The runtime is multithreaded, enables IO and Time features, and enters the runtime context.
pub fn default_runtime() -> Result<InfraRuntime, InfraError> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .map_err(|_err| InfraError::CannotStartRuntime)?;
    let _guard = runtime.enter();

    Ok(InfraRuntime::init(runtime))
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
