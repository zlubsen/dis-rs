use crate::core::{
    check_spec_for_duplicate_node_names, NodeConstructor, NodeConstructorPointer, UntypedNode,
};
use crate::error::{CreationError, ExecutionError, GatewayError, SpecificationError};
use futures::future::JoinAll;
use futures::stream::FuturesUnordered;
use std::any::Any;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use tokio::runtime::Runtime;
use tokio::signal;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::task::JoinHandle;
use tracing::{error, trace};

const COMMAND_CHANNEL_CAPACITY: usize = 50;
const EVENT_CHANNEL_CAPACITY: usize = 50;

/// The `InfraBuilder` is used to construct a specific infrastructure using composable Nodes, connected through Channels.
/// It also provisions the generic communication channels for the infrastructure.
pub struct InfraBuilder {
    command_tx: Sender<Command>,
    event_tx: Sender<Event>,
    external_input_async: Option<Box<dyn Any>>,
    external_output_async: Option<Box<dyn Any>>,
    external_channels: ExternalChannels,
    nodes: Vec<UntypedNode>,
    node_factories: Vec<(&'static str, NodeConstructor)>,
}

impl Default for InfraBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl InfraBuilder {
    /// Creates a new `InfraBuilder` environment, initialising modules.
    /// The needed coordination channels (for sending `Command`s and receiving `Event`s) are created using this constructor function.
    pub fn new() -> Self {
        let (command_tx, _command_rx) = tokio::sync::broadcast::channel(COMMAND_CHANNEL_CAPACITY);
        let (event_tx, _event_rx) = tokio::sync::broadcast::channel(EVENT_CHANNEL_CAPACITY);

        let node_factory = crate::core::builtin_nodes();

        Self {
            command_tx,
            event_tx,
            external_input_async: None,
            external_output_async: None,
            external_channels: ExternalChannels::default(),
            nodes: vec![],
            node_factories: node_factory,
        }
    }

    /// Adds the given `NodeConstructorTuple` pointer to the index of `NodeConstructor`s of this runtime.
    pub fn register_node_type(
        &mut self,
        pointer: NodeConstructorPointer,
    ) -> Result<(), GatewayError> {
        if !self
            .node_factories
            .iter()
            .any(|existing| existing.0 == pointer.0)
        {
            self.node_factories.push(pointer);
            Ok(())
        } else {
            Err(GatewayError::Registration(pointer.0.to_string()))
        }
    }

    /// Adds the given list of `NodeConstructorTuple` pointers to the index of `NodeConstructor`s of this runtime.
    pub fn register_module(
        &mut self,
        pointers: Vec<NodeConstructorPointer>,
    ) -> Result<(), GatewayError> {
        for pointer in pointers {
            self.register_node_type(pointer)?
        }
        Ok(())
    }

    /// Builds an infra specification from a given `Path`, pointing to a TOML file
    pub fn build_from_path(&mut self, path: &Path) -> Result<(), GatewayError> {
        let contents = read_to_string(path)
            .map_err(|err| GatewayError::from(SpecificationError::from(err)))?;
        self.build_spec(&contents)
    }

    /// Builds an infra specification, provided as the bare content (in TOML format)
    pub fn build_from_str(&mut self, toml_spec: &str) -> Result<(), GatewayError> {
        self.build_spec(toml_spec)
    }

    /// Builds the provided specification.
    fn build_spec(&mut self, spec: &str) -> Result<(), GatewayError> {
        let contents: toml::Table = toml::from_str(spec)
            .map_err(|err| GatewayError::from(SpecificationError::from(err)))?;

        // Check for duplicate names in the spec
        check_spec_for_duplicate_node_names(&contents)?;

        // Construct all nodes in the spec as a Vec<Box<dyn NodeData>>, giving them a unique id (index in the vec).
        let mut nodes: Vec<UntypedNode> = crate::core::construct_nodes_from_spec(
            &self.node_factories,
            self.command_tx.clone(),
            self.event_tx.clone(),
            &contents,
        )
        .map_err(GatewayError::from)?;

        // Construct all edges between the nodes from the spec.
        crate::core::register_channels_for_nodes(&contents, &mut nodes)?;

        let external_channels = ExternalChannels::new_from_nodes(&mut nodes);
        self.external_channels = external_channels;

        // FIXME remove this way of only having one external in- and output.
        // Connect optional channels to an external sender/receiver.
        // let (incoming, outgoing) = crate::core::register_external_channels(&contents, &mut nodes)?;
        // self.external_input_async = incoming;
        // self.external_output_async = outgoing;

        self.nodes = nodes;

        Ok(())
    }

    /// Obtain a sender handle to the command channel.
    pub fn command_channel(&self) -> Sender<Command> {
        self.command_tx.clone()
    }

    /// Obtain a receiver handle to the event channel.
    pub fn event_channel(&self) -> Receiver<Event> {
        self.event_tx.subscribe()
    }

    /// Returns a sender handle to the external input channel, if present (e.g., when defined in the specification).
    pub fn external_input(&mut self) -> Option<Box<dyn Any>> {
        self.external_input_async.take()
    }

    /// Returns a reference to the external output channel, if present (e.g., when defined in the specification).
    /// The reference must be used to downcast the dyn type and subscribe to the channel.
    pub fn external_output(&mut self) -> Option<Box<dyn Any>> {
        self.external_output_async.take()
    }

    pub fn external_input_for_node<T: 'static>(
        &mut self,
        node_name: &str,
    ) -> Result<Sender<T>, GatewayError> {
        self.external_channels.input_for_name::<T>(node_name)
    }

    pub fn external_output_for_node<T: 'static>(
        &mut self,
        node_name: &str,
    ) -> Result<Receiver<T>, GatewayError> {
        self.external_channels.output_for_name::<T>(node_name)
    }
}

/// Execute an infrastructure, as constructed by a `InfraRuntimeBuilder`.
pub async fn run_from_builder(
    builder: InfraBuilder,
) -> Result<JoinAll<JoinHandle<()>>, CreationError> {
    // Spawn the coordination tasks: shutdown signal and error event listeners
    let shutdown_handle = tokio::spawn(shutdown_signal(builder.command_tx.clone()));
    let event_listener_handle = tokio::spawn(event_listener(
        builder.event_tx.subscribe(),
        builder.command_tx.clone(),
    ));

    // Spawn all nodes by iterating the vec, collecting JoinHandles in a FuturesUnordered, or an error when a node failed to start
    let handles: Result<FuturesUnordered<_>, _> = builder
        .nodes
        .into_iter()
        .map(|node| node.spawn_into_runner())
        .collect();
    let handles = handles?; // Propagate any error

    // Push coordination handles to the FuturesUnordered
    handles.push(shutdown_handle);
    handles.push(event_listener_handle);

    // 7. Return the FuturesUnordered in a JoinAll so the callee can await the termination
    Ok(futures::future::join_all(handles))
}

/// General task that listens to emitted `Event`s.
///
/// This task is responsible for outputting any errors, and cleaning up the Runtime in such a case.
async fn event_listener(mut event_rx: Receiver<Event>, command_tx: Sender<Command>) {
    let mut command_rx = command_tx.subscribe();
    loop {
        tokio::select! {
            Ok(event) = event_rx.recv() => {
                match event {
                    Event::RuntimeError(err) => {
                        error!("{err}");
                        let _ = command_tx.send(Command::Quit);
                    }
                    Event::SendStatistics(stats) => {
                        trace!("{:?}", stats);
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
                        error!("{err}");
                        break;
                    }
                }
            }
        }
    }
}

/// General task that is spawned to listen for `Ctrl+C` and shutdown signals from the OS.
/// When a shutdown signal is detected, a `Command::Quit` command will be issued to all tasks listening for `Command`s.
async fn shutdown_signal(cmd: Sender<Command>) {
    // TODO remove .expect() calls, make future fallible
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let sigint = async {
        signal::unix::signal(signal::unix::SignalKind::interrupt())
            .expect("failed to install signal interrupt handler")
            .recv()
            .await;
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal terminate handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = async {
        let mut signal =
            signal::windows::ctrl_close().expect("failed to install signal ctrl_close handler");
        signal.recv().await;
    };

    #[cfg(not(unix))]
    let sigint = std::future::pending::<()>();

    let mut cmd_rx = cmd.subscribe();
    tokio::select! {
        Ok(Command::Quit) = cmd_rx.recv() => { },
        _ = ctrl_c => {  },
        _ = terminate => {  },
        _ = sigint => {  },
    }

    trace!("Shutdown signal detected, stopping the infrastructure.");
    let _ = cmd.send(Command::Quit);
}

/// Creates a default tokio runtime.
///
/// The runtime is multithreaded, enables IO and Time features, and enters the runtime context.
pub fn default_tokio_runtime() -> Result<Runtime, GatewayError> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .map_err(GatewayError::from)?;
    let _guard = runtime.enter();

    Ok(runtime)
}

/// Create an `InfraBuilder` from a spec, in TOML as a `&str`, and return all coordination and I/O channels.
///
/// # Example
/// ```ignore
/// use gateway_core::runtime::preset_builder_from_spec_str;
///
/// let Ok((builder, cmd_tx, event_rx, input_tx, output_rx)) =
///     preset_builder_from_spec_str::<InputType, OutputType>(r#"Here be some TOML contents"#);
/// ```
#[allow(clippy::type_complexity)]
pub fn preset_builder_from_spec_str<I: 'static, O: 'static>(
    toml_spec: &str,
) -> Result<
    (
        InfraBuilder,
        Sender<Command>,
        Receiver<Event>,
        Option<Sender<I>>,
        Option<Receiver<O>>,
    ),
    GatewayError,
> {
    let builder = InfraBuilder::new();
    preset_from_spec_str(builder, toml_spec)
}

/// Takes an initialised `InfraBuilder` and a specification, in TOML as a `&str`, and return all coordination and I/O channels.
///
/// # Example
/// ```ignore
/// use gateway_core::runtime::preset_from_spec_str;
///
/// let builder = InfraBuilder::new();
/// let Ok((builder, cmd_tx, event_rx, input_tx, output_rx)) =
///     preset_from_spec_str::<InputType, OutputType>(builder, r#"Here be some TOML contents"#);
/// ```
#[allow(clippy::type_complexity)]
pub fn preset_from_spec_str<I: 'static, O: 'static>(
    mut builder: InfraBuilder,
    toml_spec: &str,
) -> Result<
    (
        InfraBuilder,
        Sender<Command>,
        Receiver<Event>,
        Option<Sender<I>>,
        Option<Receiver<O>>,
    ),
    GatewayError,
> {
    builder.build_from_str(toml_spec)?;

    let cmd_tx = builder.command_channel();
    let event_rx = builder.event_channel();

    let input_tx = downcast_external_input::<I>(builder.external_input());
    let output_rx = downcast_external_output::<O>(builder.external_output());

    Ok((builder, cmd_tx, event_rx, input_tx, output_rx))
}

/// Enum of Commands that the Runtime can send to Nodes
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Quit,
}

/// Enum of Events that the Runtime and Nodes can emit
#[derive(Clone, Debug)]
pub enum Event {
    RuntimeError(ExecutionError),
    SendStatistics(String),
}

/// Convenience function to downcast an external incoming channel to a concrete, unboxed type.
///
/// The function discards any downcast errors.
pub fn downcast_external_input<T: 'static>(channel: Option<Box<dyn Any>>) -> Option<Sender<T>> {
    channel.and_then(|input| input.downcast::<Sender<T>>().map(|sender| *sender).ok())
}

/// Convenience function to downcast an external outgoing channel to a concrete, unboxed type.
///
/// The function discards any downcast errors.
pub fn downcast_external_output<T: 'static>(channel: Option<Box<dyn Any>>) -> Option<Receiver<T>> {
    channel.and_then(|input| {
        input
            .downcast::<Receiver<T>>()
            .map(|receiver| *receiver)
            .ok()
    })
}

#[derive(Default)]
pub struct ExternalChannels {
    in_map: HashMap<String, Box<dyn Any>>,
    out_map: HashMap<String, Box<dyn Any>>,
}

impl ExternalChannels {
    pub fn new_from_nodes(nodes: &mut Vec<UntypedNode>) -> Self {
        let mut in_map = HashMap::new();
        let mut out_map = HashMap::new();

        nodes.iter_mut().for_each(|node| {
            let input = node.request_external_input_sender().expect("Node exists");
            let output = node.request_external_output_sender();
            in_map.insert(node.name().to_string(), input);
            out_map.insert(node.name().to_string(), output);
        });

        Self { in_map, out_map }
    }

    // pub(crate) fn push_input(
    //     &mut self,
    //     node_name: &str,
    //     sender: Box<dyn Any>,
    // ) -> Result<(), GatewayError> {
    //     if self.in_map.insert(node_name.to_string(), sender).is_none() {
    //         Ok(())
    //     } else {
    //         Err(GatewayError::Specification(
    //             SpecificationError::DuplicateNodeNames(node_name.to_string()),
    //         ))
    //     }
    // }
    //
    // pub(crate) fn push_output(
    //     &mut self,
    //     node_name: &str,
    //     sender: Box<dyn Any>,
    // ) -> Result<(), GatewayError> {
    //     if self.out_map.insert(node_name.to_string(), sender).is_none() {
    //         Ok(())
    //     } else {
    //         Err(GatewayError::Specification(
    //             SpecificationError::DuplicateNodeNames(node_name.to_string()),
    //         ))
    //     }
    // }

    pub fn input_for_name<T: 'static>(
        &mut self,
        node_name: &str,
    ) -> Result<Sender<T>, GatewayError> {
        if let Some(boxed_node) = self.in_map.get(node_name) {
            if let Some(node) = boxed_node.downcast_ref::<Sender<T>>() {
                Ok(node.clone())
            } else {
                Err(GatewayError::Creation(
                    CreationError::SubscribeToExternalChannelWrongDataType(node_name.to_string()),
                ))
            }
        } else {
            Err(GatewayError::Creation(
                CreationError::SubscribeToExternalChannelNodeNonexistent(node_name.to_string()),
            ))
        }
    }

    pub fn output_for_name<T: 'static>(
        &mut self,
        node_name: &str,
    ) -> Result<Receiver<T>, GatewayError> {
        if let Some(boxed_node) = self.out_map.get(node_name) {
            if let Some(node) = boxed_node.downcast_ref::<Sender<T>>() {
                Ok(node.subscribe())
            } else {
                Err(GatewayError::Creation(
                    CreationError::SubscribeToExternalChannelWrongDataType(node_name.to_string()),
                ))
            }
        } else {
            Err(GatewayError::Creation(
                CreationError::SubscribeToExternalChannelNodeNonexistent(node_name.to_string()),
            ))
        }
    }
}
