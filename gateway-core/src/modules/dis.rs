use crate::core::{
    BaseNode, BaseStatistics, InstanceId, NodeConstructor, NodeConstructorPointer, NodeData,
    NodeRunner, UntypedNode, DEFAULT_AGGREGATE_STATS_INTERVAL_MS, DEFAULT_NODE_CHANNEL_CAPACITY,
    DEFAULT_OUTPUT_STATS_INTERVAL_MS,
};
use crate::error::{CreationError, ExecutionError, NodeError, SpecificationError};
use crate::node_data_impl;
use crate::runtime::{Command, Event};
use bytes::{Bytes, BytesMut};
use dis_rs::enumerations::ProtocolVersion;
use dis_rs::model::Pdu;
use serde_derive::{Deserialize, Serialize};
use std::any::Any;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::task::JoinHandle;

const SPEC_DIS_RECEIVER_NODE_TYPE: &str = "dis_receiver";
const SPEC_DIS_SENDER_NODE_TYPE: &str = "dis_sender";

const DEFAULT_SERIALISE_BUFFER_CAPACITY: usize = 32_768;

pub fn available_nodes() -> Vec<NodeConstructorPointer> {
    let dis_nodes_constructor: NodeConstructor = node_from_spec;

    let items = vec![
        (SPEC_DIS_RECEIVER_NODE_TYPE, dis_nodes_constructor),
        (SPEC_DIS_SENDER_NODE_TYPE, dis_nodes_constructor),
    ];
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
        SPEC_DIS_RECEIVER_NODE_TYPE => {
            let node = DisRxNodeData::new(instance_id, cmd_rx, event_tx, spec)?.to_dyn();
            Ok(node)
        }
        SPEC_DIS_SENDER_NODE_TYPE => {
            let node = DisTxNodeData::new(instance_id, cmd_rx, event_tx, spec)?.to_dyn();
            Ok(node)
        }
        unknown_value => Err(SpecificationError::UnknownNodeTypeForModule {
            node_type: unknown_value.to_string(),
            module_name: "dis",
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisRxNodeSpec {
    name: String,
    exercise_id: Option<u8>,
    allow_dis_versions: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct DisRxNodeData {
    base: BaseNode,
    exercise_id: Option<u8>,
    allow_dis_versions: Vec<ProtocolVersion>,
    incoming: Option<Receiver<Bytes>>,
    outgoing: Sender<Pdu>,
}

pub struct DisRxNodeRunner {
    instance_id: InstanceId,
    name: String,
    exercise_id: Option<u8>,
    allow_dis_versions: Vec<ProtocolVersion>,
    statistics: DisStatistics,
}

#[derive(Copy, Clone, Debug, Default, Serialize)]
pub struct DisStatistics {
    base: BaseStatistics,
    // TODO stats specific to DIS
}

#[derive(Debug, Error)]
pub enum DisNodeError {
    #[error("The Exercise ID must be withing 1-128, but is {0}.")]
    InvalidExerciseId(u8),
}

impl NodeError for DisNodeError {}

impl DisStatistics {
    fn new(node_id: InstanceId) -> Self {
        Self {
            base: BaseStatistics::new(node_id),
        }
    }

    fn received_incoming(&mut self) {
        self.base.incoming_message();
    }

    fn sent_outgoing(&mut self) {
        self.base.outgoing_message();
    }

    // TODO parsed/serialised PDU, failures, rejected/filtered PDUs,
}

impl NodeData for DisRxNodeData {
    fn new(
        instance_id: InstanceId,
        cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        spec: &toml::Table,
    ) -> Result<Self, SpecificationError> {
        let node_spec: DisRxNodeSpec =
            toml::from_str(&spec.to_string()).map_err(SpecificationError::ParseSpecification)?;

        let (out_tx, _out_rx) = channel(DEFAULT_NODE_CHANNEL_CAPACITY);

        let exercise_id = if let Some(id) = node_spec.exercise_id {
            if (1..=u8::MAX).contains(&id) {
                Some(id)
            } else {
                return Err(SpecificationError::Module(Box::new(
                    DisNodeError::InvalidExerciseId(id),
                )));
            }
        } else {
            None
        };

        let allow_dis_versions = node_spec
            .allow_dis_versions
            .clone()
            .map(|versions| {
                versions
                    .iter()
                    .map(|&version| ProtocolVersion::from(version))
                    .collect()
            })
            .unwrap_or(dis_rs::supported_protocol_versions());

        Ok(Self {
            base: BaseNode {
                instance_id,
                name: node_spec.name.clone(),
                cmd_rx,
                event_tx,
            },
            exercise_id,
            allow_dis_versions,
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
        DisRxNodeRunner
    );
}

impl NodeRunner for DisRxNodeRunner {
    type Data = DisRxNodeData;
    type Incoming = Bytes;
    type Outgoing = Pdu;

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
            exercise_id: data.exercise_id,
            allow_dis_versions: data.allow_dis_versions,
            statistics: DisStatistics::new(data.base.instance_id),
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
        let mut aggregate_stats_interval =
            tokio::time::interval(Duration::from_millis(DEFAULT_AGGREGATE_STATS_INTERVAL_MS));
        let mut output_stats_interval =
            tokio::time::interval(Duration::from_millis(DEFAULT_OUTPUT_STATS_INTERVAL_MS));

        loop {
            tokio::select! {
                // receiving commands
                Ok(cmd) = cmd_rx.recv() => {
                    if cmd == Command::Quit { break; }
                }
                // receiving from the incoming channel, parse into PDU
                Some(message) = Self::receive_incoming(self.instance_id, &mut incoming) => {
                    let pdus = match dis_rs::parse(&message) {
                        Ok(vec) => { vec }
                        Err(err) => {
                            Self::emit_event(&event_tx,
                                Event::RuntimeError(ExecutionError::NodeExecution {
                                    node_id: self.id(),
                                    message: format!("DIS parse error: {err}")
                                }));
                            vec![]
                        }
                    };
                    self.statistics.received_incoming();

                    pdus.into_iter()
                        .filter(|pdu| self.allow_dis_versions.contains(&pdu.header.protocol_version))
                        .filter(|pdu| self.exercise_id.is_none() || self.exercise_id.is_some_and(|exercise_id| pdu.header.exercise_id == exercise_id ))
                        .for_each(|pdu| {
                            let _send_result = outgoing.send(pdu.clone());
                            self.statistics.sent_outgoing();
                        });
                }
                // aggregate statistics for the interval
                _ = aggregate_stats_interval.tick() => {
                    self.statistics.base.aggregate_interval();
                }
                // output current state of the stats
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

#[derive(Debug, Serialize, Deserialize)]
pub struct DisTxNodeSpec {
    name: String,
    buffer_size: Option<usize>,
}

#[derive(Debug)]
pub struct DisTxNodeData {
    base: BaseNode,
    buffer: BytesMut,
    incoming: Option<Receiver<Pdu>>,
    outgoing: Sender<Bytes>,
}

pub struct DisTxNodeRunner {
    instance_id: InstanceId,
    name: String,
    buffer: BytesMut,
    statistics: DisStatistics,
}

impl NodeData for DisTxNodeData {
    fn new(
        instance_id: InstanceId,
        cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        spec: &toml::Table,
    ) -> Result<DisTxNodeData, SpecificationError> {
        let node_spec: DisTxNodeSpec =
            toml::from_str(&spec.to_string()).map_err(SpecificationError::ParseSpecification)?;

        let (out_tx, _out_rx) = channel(DEFAULT_NODE_CHANNEL_CAPACITY);

        let buffer_size = node_spec
            .buffer_size
            .unwrap_or(DEFAULT_SERIALISE_BUFFER_CAPACITY);
        let buffer = BytesMut::with_capacity(buffer_size);

        Ok(Self {
            base: BaseNode {
                instance_id,
                name: node_spec.name.clone(),
                cmd_rx,
                event_tx,
            },
            buffer,
            incoming: None,
            outgoing: out_tx,
        })
    }

    node_data_impl!(
        Pdu,
        self.incoming,
        self.outgoing,
        self.base.instance_id,
        self.base.name,
        DisTxNodeRunner
    );
}

impl NodeRunner for DisTxNodeRunner {
    type Data = DisTxNodeData;
    type Incoming = Pdu;
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
            buffer: data.buffer,
            statistics: DisStatistics::new(data.base.instance_id),
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
        let mut aggregate_stats_interval =
            tokio::time::interval(Duration::from_millis(DEFAULT_AGGREGATE_STATS_INTERVAL_MS));
        let mut output_stats_interval =
            tokio::time::interval(Duration::from_millis(DEFAULT_OUTPUT_STATS_INTERVAL_MS));

        loop {
            tokio::select! {
                // receiving commands
                Ok(cmd) = cmd_rx.recv() => {
                    if cmd == Command::Quit { break; }
                }
                // receiving from the incoming channel, serialise PDU into Bytes
                Some(message) = Self::receive_incoming(self.instance_id, &mut incoming) => {
                    self.statistics.received_incoming();
                    match message.serialize(&mut self.buffer) {
                        Ok(bytes_written) => {
                            let _send_result = outgoing
                            .send(Bytes::copy_from_slice(&self.buffer[0..(bytes_written as usize)]))
                            .inspect(|_bytes_send| self.statistics.sent_outgoing() )
                            .inspect_err(|_| {
                                Self::emit_event(&event_tx,
                                    Event::RuntimeError(ExecutionError::OutputChannelSend(self.instance_id))
                                );}
                            );
                        }
                        Err(err) => {
                            Self::emit_event(
                                &event_tx,
                                Event::RuntimeError(
                                    ExecutionError::NodeExecution {
                                        node_id: self.id(),
                                        message: err.to_string(),
                                    }
                                )
                            );
                        }
                    }
                }
                // aggregate statistics for the interval
                _ = aggregate_stats_interval.tick() => {
                    self.statistics.base.aggregate_interval();
                }
                // output current state of the stats
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
