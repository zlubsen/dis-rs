use std::any::{type_name, Any};
use std::ops::Deref;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::signal;
use tokio::task::{JoinSet};
use crate::core::{NodeData, NodeOneData, NodeOneRunner, NodeRunner, NodeTwoData, NodeTwoRunner};
use crate::error::InfraError;

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
        let (command_tx, command_rx) = tokio::sync::broadcast::channel(100);
        let (event_tx, event_rx) = tokio::sync::mpsc::channel(100);

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
        self.async_runtime.block_on( async {
            // let mut node_one_data: Box<dyn NodeData> = Box::new(NodeOneData::new(1, self.command_tx.subscribe(), self.event_tx.clone()));
            // let mut node_two_data: Box<dyn NodeData> = Box::new(NodeTwoData::new(2, self.command_tx.subscribe(), self.event_tx.clone()));
            let mut node_one_data = Box::new(NodeOneData::new(1, self.command_tx.subscribe(), self.event_tx.clone()));
            let mut node_two_data = Box::new(NodeTwoData::new(2, self.command_tx.subscribe(), self.event_tx.clone()));

            let (node_one_input_tx, node_one_input_rx) = tokio::sync::broadcast::channel::<u8>(10);
            let dyn_tx: Box<dyn Any> = Box::new(node_one_input_rx);
            node_one_data.register_subscription(dyn_tx);

            let node_one_receiver = node_one_data.request_subscription();
            node_two_data.register_subscription(node_one_receiver);

            let node_two_receiver = node_two_data.request_subscription();
            let mut node_two_receiver = if let Ok(receiver) = node_two_receiver.downcast::<tokio::sync::broadcast::Receiver<u16>>() {
                *receiver
            } else { panic!("Downcast error") };

            let node_one = tokio::spawn(async move { NodeOneRunner::with_data(node_one_data).unwrap().run().await });
            let node_two = tokio::spawn(async move { NodeTwoRunner::with_data(node_two_data).unwrap().run().await });

            loop {
                tokio::select! {
                    _ = shutdown_signal() => {
                        println!("\nReceived shutdown signal from terminal");
                        break;
                    }
                    _ = tokio::time::sleep(Duration::from_secs(15)) => {
                        println!("Timeout, shutting down");
                        break;
                    }
                    _ = tokio::time::sleep(Duration::from_secs(3)) => {
                        match node_one_input_tx.send(123) {
                            Ok(a) => {
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

            self.command_tx.send(Command::Quit).expect("error sending kill signal");

            println!("await join udp_node");
            // udp_node.await.expect("error awaiting udp_node");
            node_one.await.expect("error awaiting udp_node");
            println!("await join filter_node");
            // filter_node.await.expect("error awaiting filter_node");
            node_two.await.expect("error awaiting filter_node");

            println!("done!");
        } );
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
        .build().map_err(|err| InfraError::RuntimeCannotStart(err))?;
    let _guard = runtime.enter();

    Ok(InfraRuntime::init(runtime))
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    // RequestSubscription{ addressed_node_id: u8, callback: tokio::sync::oneshot::Sender<tokio::sync::broadcast::Receiver<Box<dyn Any>>> },
    // RegisterSubscription{ addressed_node_id: u8, channel: tokio::sync::broadcast::Receiver<Box<dyn Any>> },
    Quit
}

#[derive(Clone)]
pub enum Event {
    SendStatistics
}