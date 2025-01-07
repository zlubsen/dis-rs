use crate::core::{GenericNode, NodeConstructor, NodeData};
use crate::error::InfraError;
use crate::infra::{builtin_nodes, register_channel_from_spec};
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
    event_tx: tokio::sync::broadcast::Sender<Event>,
    node_factories: Vec<(&'static str, NodeConstructor)>,
}

impl InfraRuntime {
    /// Initialise a `InfraRuntime` environment, using the provided `tokio::Runtime`.
    /// The needed communication channels are created using this function.
    pub fn init(runtime: Runtime) -> Self {
        let (command_tx, _command_rx) = tokio::sync::broadcast::channel(COMMAND_CHANNEL_CAPACITY);
        let (event_tx, _event_rx) = tokio::sync::broadcast::channel(EVENT_CHANNEL_CAPACITY);

        let mut node_factory = Vec::new();
        node_factory.extend(builtin_nodes());

        Self {
            async_runtime: runtime,
            command_tx,
            event_tx,
            node_factories: node_factory,
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

        self.async_runtime.block_on(async {
            let contents: toml::Table =
                toml::from_str(spec).map_err(|err| InfraError::InvalidSpec {
                    message: err.to_string(),
                })?;

            // 2. Get a list of all the nodes
            // 3. Construct all nodes as a Vec<Box<dyn NodeData>>, giving them a unique id (index of the vec).
            let mut nodes: Vec<GenericNode> = self.construct_nodes_from_spec(&contents)?;

            // 4. Get a list of all the edges (channels)
            // 5. Construct all edges by getting the nodes from the vec.
            Self::register_channels_for_nodes(&contents, &mut nodes)?;

            // 6. Spawn all nodes by iterating the vec, collecting JoinHandles in a JoinSet(?)
            let handles = nodes
                .into_iter()
                .map(|node| node.spawn_into_runner())
                .collect::<FuturesUnordered<_>>();

            // Coordination: shutdown signal and error even listeners
            handles.push(tokio::spawn(shutdown_signal(self.command_tx.clone())));

            handles.push(tokio::spawn(event_listener(
                self.event_tx.subscribe(),
                self.command_tx.clone(),
            )));

            // 7. Wait for all tasks to finish
            let _ = futures::future::join_all(handles).await;

            Ok(())
        })
    }

    fn lookup_node_constructor(&self, node_type: &str) -> Result<NodeConstructor, InfraError> {
        self.node_factories
            .iter()
            .find(|&&tup| tup.0 == node_type)
            .ok_or(InfraError::InvalidSpec {
                message: format!("Node type {node_type} is not known."),
            })
            .map(|tup| tup.1)
    }

    fn construct_nodes_from_spec(
        &self,
        contents: &toml::Table,
    ) -> Result<Vec<GenericNode>, InfraError> {
        if let Value::Array(array) = &contents["nodes"] {
            let nodes: Vec<Result<GenericNode, InfraError>> = array
                .iter()
                .enumerate()
                .map(|(id, node)| {
                    if let Value::Table(spec) = node {
                        let factory =
                            self.lookup_node_constructor(spec["type"].as_str().unwrap_or_default());

                        let factory = factory?;
                        match factory(
                            id as u64,
                            self.command_tx.subscribe(),
                            self.event_tx.clone(),
                            spec,
                        ) {
                            Ok(node) => Ok(node),
                            Err(err) => Err(err),
                        }
                    } else {
                        Err(InfraError::InvalidSpec {
                            message: format!("Invalid node spec for index {id}"),
                        })
                    }
                })
                .collect();
            let nodes: Result<Vec<GenericNode>, InfraError> = nodes.into_iter().collect();
            nodes
        } else {
            Err(InfraError::InvalidSpec {
                message: "A spec file must contain a non-empty list of 'nodes', which is missing"
                    .to_string(),
            })
        }
    }

    fn register_channels_for_nodes(
        contents: &toml::Table,
        nodes: &mut Vec<GenericNode>,
    ) -> Result<(), InfraError> {
        if let Value::Array(array) = &contents["channels"] {
            for channel in array {
                if let Value::Table(spec) = channel {
                    if let Err(err) = register_channel_from_spec(spec, nodes) {
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
        Ok(())
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
