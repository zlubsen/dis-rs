use crate::core::{
    BaseNode, BaseNodeSpec, BaseStatistics, InstanceId, NodeConstructor, NodeData, NodeRunner,
    UntypedNode, DEFAULT_AGGREGATE_STATS_INTERVAL_MS, DEFAULT_NODE_CHANNEL_CAPACITY,
    DEFAULT_OUTPUT_STATS_INTERVAL_MS,
};
use crate::error::{CreationError, ExecutionError, SpecificationError};
use crate::node_data_impl;
use crate::runtime::{Command, Event};
use bytes::Bytes;
use std::any::Any;
use std::time::Duration;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::task::JoinHandle;

const SPEC_PASS_THROUGH_NODE_TYPE: &str = "pass_through";

pub fn available_nodes() -> Vec<(&'static str, NodeConstructor)> {
    let util_nodes_constructor: NodeConstructor = node_from_spec;

    let items = vec![(SPEC_PASS_THROUGH_NODE_TYPE, util_nodes_constructor)];
    items
}

pub fn node_from_spec(
    instance_id: InstanceId,
    cmd_rx: Receiver<Command>,
    event_tx: Sender<Event>,
    type_value: &str,
    spec: &toml::Table,
) -> Result<UntypedNode, SpecificationError> {
    match type_value {
        SPEC_PASS_THROUGH_NODE_TYPE => {
            let node = PassThroughNodeData::new(instance_id, cmd_rx, event_tx, spec)?.to_dyn();
            Ok(node)
        }
        unknown_value => Err(SpecificationError::UnknownNodeTypeForModule {
            node_type: unknown_value.to_string(),
            module_name: "util",
        }),
    }
}

#[derive(Debug)]
pub struct PassThroughNodeData {
    base: BaseNode,
    incoming: Option<Receiver<Bytes>>,
    outgoing: Sender<Bytes>,
}

pub struct PassThroughNodeRunner {
    instance_id: InstanceId,
    name: String,
    statistics: BaseStatistics,
}

impl NodeData for PassThroughNodeData {
    fn new(
        instance_id: InstanceId,
        cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        spec: &toml::Table,
    ) -> Result<PassThroughNodeData, SpecificationError> {
        let (out_tx, _out_rx) = channel(DEFAULT_NODE_CHANNEL_CAPACITY);

        let node_spec: BaseNodeSpec =
            toml::from_str(&spec.to_string()).map_err(SpecificationError::ParseSpecification)?;

        Ok(Self {
            base: BaseNode {
                instance_id,
                name: node_spec.name.clone(),
                cmd_rx,
                event_tx,
            },
            incoming: None,
            outgoing: out_tx,
        })
    }

    node_data_impl!(
        Bytes,
        self.incoming,
        self.outgoing,
        self.base.instance_id,
        self.base.name,
        PassThroughNodeRunner
    );
}

impl NodeRunner for PassThroughNodeRunner {
    type Data = PassThroughNodeData;
    type Incoming = Bytes;
    type Outgoing = Bytes;

    fn id(&self) -> InstanceId {
        self.instance_id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn spawn_with_data(data: Self::Data) -> Result<JoinHandle<()>, CreationError> {
        let mut node_runner = Self {
            instance_id: data.base.instance_id,
            name: data.base.name,
            statistics: BaseStatistics::default(),
        };

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

    async fn run(
        &mut self,
        mut cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        mut incoming: Option<Receiver<Self::Incoming>>,
        outgoing: Sender<Self::Outgoing>,
    ) {
        loop {
            let mut aggregate_stats_interval =
                tokio::time::interval(Duration::from_millis(DEFAULT_AGGREGATE_STATS_INTERVAL_MS));
            let mut output_stats_interval =
                tokio::time::interval(Duration::from_millis(DEFAULT_OUTPUT_STATS_INTERVAL_MS));

            tokio::select! {
                // receiving commands
                Ok(cmd) = cmd_rx.recv() => {
                    if cmd == Command::Quit { break; }
                }
                Some(message) = Self::receive_incoming(self.instance_id, &mut incoming) => {
                    let _send_result = outgoing.send(message).inspect_err(|_|
                        Self::emit_event(&event_tx,
                            Event::RuntimeError(ExecutionError::OutputChannelSend(self.id())))
                    );
                }
                _ = aggregate_stats_interval.tick() => {
                    self.statistics.aggregate_interval();
                }
                _ = output_stats_interval.tick() => {
                    // TODO
                }
            }
        }
    }
}
