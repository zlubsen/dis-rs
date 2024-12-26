use crate::core::{NodeData, NodeOneData, NodeTwoData};
use crate::error::InfraError;
use std::any::Any;
use std::ops::Add;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::signal;
use tokio::task::JoinSet;
use tokio::time::Instant;

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

    pub fn run(&self) {
        self.async_runtime.block_on(async {
            let mut node_one_data: Box<dyn NodeData> = Box::new(NodeOneData::new(
                1,
                self.command_tx.subscribe(),
                self.event_tx.clone(),
            ));
            let mut node_two_data: Box<dyn NodeData> = Box::new(NodeTwoData::new(
                2,
                self.command_tx.subscribe(),
                self.event_tx.clone(),
            ));

            // connect the nodes
            let node_one_receiver = node_one_data.request_subscription();
            let _ = node_two_data.register_subscription(node_one_receiver);

            // connect the first node to an input channel, for testing
            let (node_one_input_tx, node_one_input_rx) = tokio::sync::broadcast::channel::<u8>(10);
            let dyn_tx: Box<dyn Any> = Box::new(node_one_input_rx);
            let _ = node_one_data.register_subscription(dyn_tx);

            // connect the last node to an output channel, for testing
            let node_two_receiver = node_two_data.request_subscription();
            let mut node_two_receiver = if let Ok(receiver) =
                node_two_receiver.downcast::<tokio::sync::broadcast::Receiver<u16>>()
            {
                *receiver
            } else {
                panic!("Downcast error")
            };

            let node_one = node_one_data.spawn_into_runner();
            let node_two = node_two_data.spawn_into_runner();

            let mut timeout_interval = tokio::time::interval_at(
                Instant::now().add(Duration::from_secs(15)),
                Duration::from_secs(1),
            );
            let mut input_interval = tokio::time::interval_at(
                Instant::now().add(Duration::from_secs(3)),
                Duration::from_secs(3),
            );

            loop {
                tokio::select! {
                    _ = shutdown_signal() => {
                        println!("\nReceived shutdown signal from terminal");
                        break;
                    }
                    _ = timeout_interval.tick() => {
                        println!("Timeout, shutting down");
                        break;
                    }
                    _ = input_interval.tick() => {
                        match node_one_input_tx.send(123) {
                            Ok(_num_receivers) => {
                                println!("Sent input to NodeOne (123)");
                            }
                            Err(err) => {
                                println!("{err}");
                            }
                        };
                        ()
                    }
                    Ok(output) = node_two_receiver.recv() => {
                        println!("NodeTwo output: {output}");
                    }
                }
            }

            self.command_tx
                .send(Command::Quit)
                .expect("error sending kill signal");

            println!("await join udp_node");
            // udp_node.await.expect("error awaiting udp_node");
            node_one.await.expect("error awaiting udp_node");
            println!("await join filter_node");
            // filter_node.await.expect("error awaiting filter_node");
            node_two.await.expect("error awaiting filter_node");

            println!("done!");
        });
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
