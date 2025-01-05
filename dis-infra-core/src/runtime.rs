use crate::core::NodeData;
use crate::error::InfraError;
use crate::infra::spec_to_node_data;
use std::fs::read_to_string;
use std::future::Future;
use std::path::Path;
use tokio::runtime::Runtime;
use tokio::signal;
use tokio::task::{JoinHandle, JoinSet};
use toml::Value;

const COMMAND_CHANNEL_CAPACITY: usize = 50;
const EVENT_CHANNEL_CAPACITY: usize = 50;

pub struct InfraRuntime {
    async_runtime: Runtime,
    join_set: JoinSet<()>,
    command_tx: tokio::sync::broadcast::Sender<Command>,
    command_rx: tokio::sync::broadcast::Receiver<Command>,
    event_tx: tokio::sync::mpsc::Sender<Event>,
    event_rx: tokio::sync::mpsc::Receiver<Event>,
}

impl InfraRuntime {
    pub fn init(runtime: Runtime) -> Self {
        let (command_tx, command_rx) = tokio::sync::broadcast::channel(COMMAND_CHANNEL_CAPACITY);
        let (event_tx, event_rx) = tokio::sync::mpsc::channel(EVENT_CHANNEL_CAPACITY);

        Self {
            async_runtime: runtime,
            join_set: JoinSet::new(),
            command_tx,
            command_rx,
            event_tx,
            event_rx,
        }
    }

    pub fn run_with_spec(&self, path: &Path) -> Result<(), InfraError> {
        let contents = read_to_string(path).map_err(|err| InfraError::InvalidSpec {
            message: err.to_string(),
        })?;
        self.run_with_spec_string(&contents)
    }

    pub fn run_with_spec_string(&self, spec: &str) -> Result<(), InfraError> {
        // 1. Read the meta info of the config, if any
        // 2. Get a list of all the nodes
        // 3. Construct all nodes as a Vec<Box<dyn NodeData>>, giving them a unique id (index of the vec).
        // 4. Get a list of all the edges (channels)
        // 5. Construct all edges by getting the nodes from the vec.
        // 6. Spawn all nodes by iterating the vec, collecting JoinHandles in a JoinSet(?)
        // 7. Wait for all tasks to finish (try_join)
        self.async_runtime.block_on(async {
            let contents: toml::Table =
                toml::from_str(spec).map_err(|err| InfraError::InvalidSpec {
                    message: err.to_string(),
                })?;

            let nodes: Vec<Box<dyn NodeData>> = if let Value::Array(array) = &contents["nodes"] {
                array.iter().enumerate()
                    .map(|(id, node)| {
                        if let Value::Table(spec) = node {
                            match spec_to_node_data(id as u64, self.command_tx.subscribe(), self.event_tx.clone(), spec) {
                                Ok(node) => { Ok(node) }
                                Err(err) => { return Err(err) }
                            }
                        } else { return Err(InfraError::InvalidSpec { message: format!("Invalid node spec for index {id}") }); }
                    })
                    .filter(Result::is_ok).map(|node| node.unwrap())
                    .collect()
            } else { return Err(InfraError::InvalidSpec { message: "A spec file must consist of a list of nodes and a list of edges between those nodes".to_string() }) };

            // TODO - have a runner return the run function/future, and then spawn using the JoinSet
            let handles: Vec<JoinHandle<()>> = nodes.into_iter().map(|node| node.spawn_into_runner()).collect();

            Ok(())
        })
    }

    pub fn run(&self, f: impl Future) {
        self.async_runtime.block_on(f);
        // self.async_runtime.block_on(async {
        //     let mut node_one_data: Box<dyn NodeData> = Box::new(NodeOneData::new(
        //         1,
        //         self.command_tx.subscribe(),
        //         self.event_tx.clone(),
        //     ));
        //     let mut node_two_data: Box<dyn NodeData> = Box::new(NodeTwoData::new(
        //         2,
        //         self.command_tx.subscribe(),
        //         self.event_tx.clone(),
        //     ));
        //
        //     // connect the nodes
        //     let node_one_receiver = node_one_data.request_subscription();
        //     let _ = node_two_data.register_subscription(node_one_receiver);
        //
        //     // connect the first node to an input channel, for testing
        //     let (node_one_input_tx, node_one_input_rx) = tokio::sync::broadcast::channel::<u8>(10);
        //     let dyn_tx: Box<dyn Any> = Box::new(node_one_input_rx);
        //     let _ = node_one_data.register_subscription(dyn_tx);
        //
        //     // connect the last node to an output channel, for testing
        //     let node_two_receiver = node_two_data.request_subscription();
        //     let mut node_two_receiver = if let Ok(receiver) =
        //         node_two_receiver.downcast::<tokio::sync::broadcast::Receiver<u16>>()
        //     {
        //         *receiver
        //     } else {
        //         panic!("Downcast error")
        //     };
        //
        //     let node_one = node_one_data.spawn_into_runner();
        //     let node_two = node_two_data.spawn_into_runner();
        //
        //     let mut timeout_interval = tokio::time::interval_at(
        //         Instant::now().add(Duration::from_secs(15)),
        //         Duration::from_secs(1),
        //     );
        //     let mut input_interval = tokio::time::interval_at(
        //         Instant::now().add(Duration::from_secs(3)),
        //         Duration::from_secs(3),
        //     );
        //
        //     loop {
        //         tokio::select! {
        //             _ = shutdown_signal() => {
        //                 println!("\nReceived shutdown signal from terminal");
        //                 break;
        //             }
        //             _ = timeout_interval.tick() => {
        //                 println!("Timeout, shutting down");
        //                 break;
        //             }
        //             _ = input_interval.tick() => {
        //                 match node_one_input_tx.send(123) {
        //                     Ok(_num_receivers) => {
        //                         println!("Sent input to NodeOne (123)");
        //                     }
        //                     Err(err) => {
        //                         println!("{err}");
        //                     }
        //                 };
        //                 ()
        //             }
        //             Ok(output) = node_two_receiver.recv() => {
        //                 println!("NodeTwo output: {output}");
        //             }
        //         }
        //     }
        //
        //     self.command_tx
        //         .send(Command::Quit)
        //         .expect("error sending kill signal");
        //
        //     println!("await join udp_node");
        //     // udp_node.await.expect("error awaiting udp_node");
        //     node_one.await.expect("error awaiting udp_node");
        //     println!("await join filter_node");
        //     // filter_node.await.expect("error awaiting filter_node");
        //     node_two.await.expect("error awaiting filter_node");
        //
        //     println!("done!");
        // });
    }
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

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Quit,
}

#[derive(Clone)]
pub enum Event {
    NodeError(InfraError),
    SendStatistics,
}
