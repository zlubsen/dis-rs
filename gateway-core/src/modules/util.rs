use crate::core::{
    BaseNode, BaseNodeSpec, BaseStatistics, InstanceId, NodeConstructor, NodeData, NodeRunner,
    UntypedNode, DEFAULT_AGGREGATE_STATS_INTERVAL_MS, DEFAULT_NODE_CHANNEL_CAPACITY,
    DEFAULT_OUTPUT_STATS_INTERVAL_MS,
};
use crate::error::InfraError;
use crate::runtime::{Command, Event};
use bytes::Bytes;
use std::any::Any;
use std::time::Duration;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::task::JoinHandle;

const SPEC_PASS_THROUGH_NODE_TYPE: &str = "pass_through";

pub fn available_nodes() -> Vec<(&'static str, NodeConstructor)> {
    let util_nodes_constructor: NodeConstructor = node_from_spec;

    let mut items = Vec::new();
    items.push((SPEC_PASS_THROUGH_NODE_TYPE, util_nodes_constructor));
    items
}

pub fn node_from_spec(
    instance_id: InstanceId,
    cmd_rx: Receiver<Command>,
    event_tx: Sender<Event>,
    type_value: &str,
    spec: &toml::Table,
) -> Result<UntypedNode, InfraError> {
    match type_value {
        SPEC_PASS_THROUGH_NODE_TYPE => {
            let node = PassThroughNodeData::new(instance_id, cmd_rx, event_tx, spec)?.to_dyn();
            Ok(node)
        }
        unknown_value => Err(InfraError::InvalidSpec {
            message: format!("Unknown node type '{unknown_value}' for module 'util'"),
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
    ) -> Result<Self, InfraError> {
        let (out_tx, _out_rx) = channel(DEFAULT_NODE_CHANNEL_CAPACITY);

        let node_spec: BaseNodeSpec =
            toml::from_str(&spec.to_string()).map_err(|err| InfraError::InvalidSpec {
                message: err.to_string(),
            })?;

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

    fn request_subscription(&self) -> Box<dyn Any> {
        let client = self.outgoing.subscribe();
        Box::new(client)
    }

    fn register_subscription(&mut self, receiver: Box<dyn Any>) -> Result<(), InfraError> {
        if let Ok(receiver) = receiver.downcast::<Receiver<Bytes>>() {
            self.incoming = Some(*receiver);
            Ok(())
        } else {
            Err(InfraError::SubscribeToChannel {
                instance_id: self.base.instance_id,
                node_name: self.base.name.clone(),
                data_type_expected: "Bytes".to_string(),
            })
        }
    }

    fn request_external_sender(&mut self) -> Result<Box<dyn Any>, InfraError> {
        let (incoming_tx, incoming_rx) = channel::<Bytes>(DEFAULT_NODE_CHANNEL_CAPACITY);
        self.register_subscription(Box::new(incoming_rx))?;
        Ok(Box::new(incoming_tx))
    }

    fn id(&self) -> InstanceId {
        self.base.instance_id
    }

    fn name(&self) -> &str {
        self.base.name.as_str()
    }

    fn spawn_into_runner(self: Box<Self>) -> Result<JoinHandle<()>, InfraError> {
        PassThroughNodeRunner::spawn_with_data(*self)
    }
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

    fn spawn_with_data(data: Self::Data) -> Result<JoinHandle<()>, InfraError> {
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
                    let _send_result = outgoing.send(message);
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
