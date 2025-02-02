use gateway_core::error::GatewayError;
use gateway_core::runtime;
use gateway_core::runtime::{
    default_tokio_runtime, downcast_external_input, downcast_external_output, run_from_builder,
    Command, InfraBuilder,
};
use std::time::Duration;
use tokio::time::Instant;
use tracing::level_filters::LevelFilter;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .pretty()
        // set logging level using environment variable; defaults to ERROR
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::TRACE.into())
                .from_env_lossy(),
        )
        // sets this to be the default, global collector for this application.
        .init();

    let spec = r#"
        [[ nodes ]]
        type = "mul_isize"
        name = "Mul 2"
        factor = 2

        [[ nodes ]]
        type = "mul_isize"
        name = "No factor"

        [[ channels ]]
        from = "Mul 2"
        to = "No factor"

        [ externals ]
        incoming = "Mul 2"
        outgoing = "No factor"
    "#;

    let runtime =
        default_tokio_runtime().expect("Expected tokio runtime to be created successfully.");

    let mut infra_runtime_builder = InfraBuilder::new();
    // The main difference to the basic-usage-gateway example is that we need to register our custom node(s)
    // This can be done for the whole module, or nodes separately via `builder.register_node_type(...)`.
    if let Err(err) = infra_runtime_builder.register_module(example_node::available_nodes()) {
        error!("{err}");
    }

    let (infra_runtime_builder, cmd_tx, _event_rx, input_tx, output_rx) =
        runtime::preset_from_spec_str::<isize, isize>(infra_runtime_builder, spec).unwrap();
    let input_tx = input_tx.unwrap();
    let mut output_rx = output_rx.unwrap();

    const QUIT_DELAY: u64 = 1;
    const SEND_DELAY: u64 = QUIT_DELAY / 2;

    // Now we can start the configured infrastructure using the async `run_from_builder` function, on a Tokio runtime.
    // Additional tasks need to be spawned to use the coordination and external I/O channels.
    if let Err(err) = runtime.block_on(async {
        // Send a Command::Quit after some time
        let cmd_handle = tokio::spawn(async move {
            tokio::time::interval_at(
                Instant::now() + Duration::from_secs(QUIT_DELAY),
                Duration::from_secs(1),
            )
            .tick()
            .await;
            info!("Sending the Command::Quit signal.");
            let _ = cmd_tx.send(Command::Quit);
        });

        // Send a message through the nodes after some shorter time
        // Do note that once this channel is closed the receiving node will produce errors,
        // as the receiving end of the channel is closed.
        let input_handle = tokio::spawn(async move {
            tokio::time::interval_at(
                Instant::now() + Duration::from_secs(SEND_DELAY),
                Duration::from_secs(1),
            )
            .tick()
            .await;
            let input = 10;
            info!("Sending a message through the nodes: {}.", input);
            let _ = input_tx.send(input);
            input_tx // We return the sender handle to keep the channel open.
        });

        // Await and print the resulting output
        let output_handle = tokio::spawn(async move {
            let out = output_rx.recv().await.unwrap();
            info!("Received the message: {}", out);
        });

        let join_all = run_from_builder(infra_runtime_builder).await?;
        let _runtime_result = join_all.await;

        let _ = input_handle.await;
        let _ = output_handle.await;
        let _ = cmd_handle.await;

        Ok::<(), GatewayError>(())
    }) {
        error!("{err}");
    }
}

/// This module demonstrates how to create a module that provides a single Node to use in a gateway.
/// In this example we have one Node `type`: `mul_isize`, for a Node that takes an isize input value,
/// multiplies by a specified value and forwards the result.
mod example_node {
    use gateway_core::core::{
        BaseNode, BaseStatistics, InstanceId, NodeConstructor, NodeConstructorPointer, NodeData,
        NodeRunner, UntypedNode, DEFAULT_AGGREGATE_STATS_INTERVAL_MS,
        DEFAULT_NODE_CHANNEL_CAPACITY, DEFAULT_OUTPUT_STATS_INTERVAL_MS,
    };
    use gateway_core::error::{CreationError, ExecutionError, SpecificationError};
    use gateway_core::node_data_impl;
    use gateway_core::runtime::{Command, Event};
    use serde::Deserialize;
    use std::any::Any;
    use std::time::Duration;
    use tokio::sync::broadcast::{channel, Receiver, Sender};
    use tokio::task::JoinHandle;

    /// A constant that defines the value of a Node's `type` field in the specification file.
    /// A module can define multiple Node types.
    const SPEC_MULTIPLY_ISIZE_NODE_TYPE: &str = "mul_isize";
    const DEFAULT_MULTIPLICATION_FACTOR: isize = 1;

    /// A module that contains a set of Nodes should provide a function `available_nodes()` that
    /// returns a `Vec` of `NodeConstructorPointer`s.
    /// Each pointer links the `type` of a Node (in the specification) to a function
    /// having signature `NodeConstructor` that can create the actual Node.
    pub fn available_nodes() -> Vec<NodeConstructorPointer> {
        let example_nodes_constructor: NodeConstructor = node_from_spec;

        let items = vec![(SPEC_MULTIPLY_ISIZE_NODE_TYPE, example_nodes_constructor)];
        items
    }

    /// The function with a signature of `NodeConstructor` must be able to construct concrete Nodes.
    /// It determines what Node to construct from the `spec` based on the `type_value`, and uses the given values
    /// `instance_id`, `cmd_rx`, and `event_tx` to wire up the node in the runtime.
    ///
    /// The Node is returned as a trait object `UntypedNode`.
    pub fn node_from_spec(
        instance_id: InstanceId,
        cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        type_value: &str,
        spec: &toml::Table,
    ) -> Result<UntypedNode, SpecificationError> {
        match type_value {
            SPEC_MULTIPLY_ISIZE_NODE_TYPE => {
                let node =
                    MultiplyIsizeNodeData::new(instance_id, cmd_rx, event_tx, spec)?.to_dyn();
                Ok(node)
            }
            unknown_value => Err(SpecificationError::UnknownNodeTypeForModule {
                node_type: unknown_value.to_string(),
                module_name: "example_node",
            }),
        }
    }

    /// Create a struct to deserialize the TOML spec part for the node into.
    /// It should at the minimum contain a field `name` of type String.
    /// If no other fields are needed, the base implementation `gateway_core::core::BaseNodeSpec` is provided
    /// In this example the factor for multiplication is optional.
    #[derive(Debug, Deserialize)]
    pub struct MultiplyIsizeNodeSpec {
        pub name: String,
        pub factor: Option<isize>,
    }

    /// Create a ...NodeData struct that is used to set up the basics of the node, based on the ...NodeSpec.
    /// For this example, beside the `BaseNode` that holds the common data fields for a Node,
    /// we have a field that holds the specified multiplication factor.
    ///
    /// The difference between the ...Spec and the ...Data is that all optional fields are either set or defaulted,
    /// and the ...Data struct holds all fields that the runtime provides the Nodes to function.
    #[derive(Debug)]
    pub struct MultiplyIsizeNodeData {
        base: BaseNode,
        factor: isize,
        incoming: Option<Receiver<isize>>,
        outgoing: Sender<isize>,
    }

    /// The ...NodeRunner struct is what is used inside the runtime. ...NodeData substructures are flattened.
    /// Specification fields are (to be) converted to the format needed at runtime,
    /// such as an IP address to a `SocketAddr`.
    ///
    /// The ...NodeRunner also adds the data structure to hold runtime statistics.
    /// Minimally this is a `BaseStatistics` struct, but could be customized to hold other statistics.
    /// It is recommended to create methods on the custom struct to handle counting stuff when major events happen,
    /// such as receiving an incoming message, sending an outgoing message, similar to how `BaseStatistics` does this.
    ///
    /// Technically the difference between a ...NodeData and ...NodeRunner is also that
    /// NodeData needs to be a trait object, which NodeRunners cannot be as they are spawned into Futures and have associated types.
    pub struct MultiplyIsizeNodeRunner {
        instance_id: InstanceId,
        name: String,
        factor: isize,
        statistics: BaseStatistics,
    }

    /// The `NodeData` trait impl allows a NodeData to be connected to
    /// other nodes based on the specification by the runtime.
    ///
    /// Much of the boilerplate for this trait impl is handled by the
    /// `node_data_impl!` macro.
    ///
    /// `node_data_impl!` takes six parameters
    /// - the data type that the node accepts
    /// - the field of the incoming channel in the NodeData struct
    /// - the field of the outgoing channel in the NodeData struct
    /// - the field of the node id / instance id in the NodeData struct
    /// - the field of the node name in the NodeData struct
    /// - the concrete type of the NodeRunner that this Node uses.
    impl NodeData for MultiplyIsizeNodeData {
        /// The `new()` associated function is used to create a valid instance of the `MultiplyIsizeNodeData`.
        /// It is to parse the specification into a ...NodeSpec struct, and with that initialise the ...NodeData struct.
        fn new(
            instance_id: InstanceId,
            cmd_rx: Receiver<Command>,
            event_tx: Sender<Event>,
            spec: &toml::Table,
        ) -> Result<MultiplyIsizeNodeData, SpecificationError> {
            let (out_tx, _out_rx) = channel(DEFAULT_NODE_CHANNEL_CAPACITY);

            let node_spec: MultiplyIsizeNodeSpec = toml::from_str(&spec.to_string())
                .map_err(SpecificationError::ParseSpecification)?;

            Ok(Self {
                base: BaseNode {
                    instance_id,
                    name: node_spec.name.clone(),
                    cmd_rx,
                    event_tx,
                },
                factor: node_spec.factor.unwrap_or(DEFAULT_MULTIPLICATION_FACTOR),
                incoming: None,
                outgoing: out_tx,
            })
        }

        node_data_impl!(
            isize,
            self.incoming,
            self.outgoing,
            self.base.instance_id,
            self.base.name,
            MultiplyIsizeNodeRunner
        );
    }

    /// The `NodeRunner` trait impl handles the execution of the Node.
    impl NodeRunner for MultiplyIsizeNodeRunner {
        type Data = MultiplyIsizeNodeData;
        type Incoming = isize;
        type Outgoing = isize;

        /// Shorthand method to access the node's Id.
        fn id(&self) -> InstanceId {
            self.instance_id
        }

        /// Shorthand method to access the node's name.
        fn name(&self) -> &str {
            &self.name
        }

        /// The `spawn_with_data` function takes the associated `NodeData` type and
        /// spawns the actual task.
        ///
        /// First the NodeData is converted into a NodeRunner, with could involve
        /// additional actions to create the runtime data structure (such as allocating buffers or what not).
        fn spawn_with_data(data: Self::Data) -> Result<JoinHandle<()>, CreationError> {
            let mut node_runner = Self {
                instance_id: data.base.instance_id,
                name: data.base.name,
                factor: data.factor,
                statistics: BaseStatistics::new(data.base.instance_id),
            };

            // Channels are passed separately from the Node's data, to avoid (mutable) borrow checker conflicts.
            Ok(tokio::spawn(async move {
                node_runner
                    .run(
                        data.base.cmd_rx,
                        data.base.event_tx,
                        data.incoming,
                        data.outgoing,
                    )
                    .await
            }))
        }

        /// The `run` method is where the actual runtime behavior of the Node is defined.
        /// It should typically loop over a `select!` that
        /// - awaits incoming messages
        /// - awaits commands (and properly stops the loop on receiving `Command::Quit`)
        /// - periodically aggregate gathered statistics
        /// - periodically output statistics via the event channel
        /// - ... does other things that are of use for the Node.
        async fn run(
            &mut self,
            mut cmd_rx: Receiver<Command>,
            event_tx: Sender<Event>,
            mut incoming: Option<Receiver<Self::Incoming>>,
            outgoing: Sender<Self::Outgoing>,
        ) {
            let mut aggregate_stats_interval =
                tokio::time::interval(Duration::from_millis(DEFAULT_AGGREGATE_STATS_INTERVAL_MS));
            let mut output_stats_interval =
                tokio::time::interval(Duration::from_millis(DEFAULT_OUTPUT_STATS_INTERVAL_MS));

            loop {
                tokio::select! {
                    // Receiving commands
                    Ok(cmd) = cmd_rx.recv() => {
                        if cmd == Command::Quit { break; }
                    }
                    // The associated function `Self::receive_incoming` is a convenience function/wrapper
                    // around the incoming channel to be used in the `select!`.
                    Some(message) = Self::receive_incoming(self.instance_id, &mut incoming) => {
                        // In this example, here is where we do the actual multiplication of the data
                        let multiplied = message * self.factor;

                        self.statistics.incoming_message();
                        let _send_result = outgoing.send(multiplied)
                            .inspect(|_| self.statistics.outgoing_message() )
                            .inspect_err(|_|
                                Self::emit_event(&event_tx,
                                    Event::RuntimeError(ExecutionError::OutputChannelSend(self.id())))
                        );
                    }
                    // Periodic aggregation of statistics
                    _ = aggregate_stats_interval.tick() => {
                        self.statistics.aggregate_interval();
                    }
                    // Periodic output of statistics
                    _ = output_stats_interval.tick() => {
                        if let Ok(json) = serde_json::to_string_pretty(&self.statistics) {
                            Self::emit_event(&event_tx,
                                Event::SendStatistics(json))
                        }
                    }
                }
            }
        }
    }
}
