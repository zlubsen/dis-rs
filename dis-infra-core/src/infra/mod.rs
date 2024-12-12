use crate::core::{BaseNode, NodeData, NodeRunner};
use crate::runtime::{Command, Event};
use bytes::{Bytes, BytesMut};
use std::any::Any;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::task::JoinHandle;

const COMMAND_CHANNEL_CAPACITY: usize = 50;
const EVENT_CHANNEL_CAPACITY: usize = 50;
const NODE_CHANNEL_CAPACITY: usize = 50;

pub struct UdpNodeData {
    base: BaseNode,
    buffer: BytesMut,
    socket_spec: String,
    incoming: Option<Receiver<Bytes>>,
    outgoing: Sender<Bytes>,
}

pub struct UdpNodeRunner {
    data: UdpNodeData,
}

impl UdpNodeData {
    pub fn new(
        instance_id: u64,
        cmd_rx: Receiver<Command>,
        event_tx: tokio::sync::mpsc::Sender<Event>,
    ) -> Self {
        let (out_tx, _out_rx) = tokio::sync::broadcast::channel(NODE_CHANNEL_CAPACITY);

        let mut buffer = BytesMut::with_capacity(32768);
        buffer.resize(32684, 0);

        Self {
            base: BaseNode {
                instance_id,
                cmd_rx,
                event_tx,
            },
            buffer,
            socket_spec: "127.0.0.1:3000".to_string(),
            incoming: None,
            outgoing: out_tx,
        }
    }
}

impl NodeData for UdpNodeData {
    fn request_subscription(&self) -> Box<dyn Any> {
        let client = self.outgoing.subscribe();
        Box::new(client)
    }

    fn register_subscription(&mut self, receiver: Box<dyn Any>) {
        if let Ok(receiver) = receiver.downcast::<Receiver<Bytes>>() {
            self.incoming = Some(*receiver);
            // Ok(())
        } else {
            // Err(InfraError::SubscribeToChannelError)
        }
    }

    fn spawn_into_runner(self: Box<Self>) -> JoinHandle<()> {
        UdpNodeRunner::spawn_with_data(*self)
    }
}

impl NodeRunner for UdpNodeRunner {
    type Data = UdpNodeData;

    fn spawn_with_data(data: Self::Data) -> JoinHandle<()> {
        let mut node_runner = Self { data };
        tokio::spawn(async move { node_runner.run().await })
    }

    async fn run(&mut self) {
        let mut collect_stats_interval = tokio::time::interval(Duration::from_secs(3));
        let socket = UdpSocket::bind(self.data.socket_spec.clone())
            .await
            .unwrap();

        // loop {
        //     select! {
        //         Ok(cmd) = self.data.base.cmd_rx.recv() => {
        //             if cmd == Command::Quit { break; }
        //             ()
        //         },
        //         Some(incoming_data) = async {
        //             match &mut self.data.incoming {
        //                 Some(channel) => channel.recv().await.ok(),
        //                 None => None,
        //             }} => {
        //             match socket.send_to(&incoming_data, "127.0.0.1:3000").await {
        //                 Ok(_) => { } // TODO did sent bytes over socket
        //                 Err(_) => { }  // TODO failed to send bytes over socket
        //             }
        //             ()
        //         },
        //         Ok(bytes_received) = socket.recv_from(&mut self.data.buffer) => {
        //             if let Ok(num_receivers) = self.data.outgoing.send(self.data.buffer[..bytes_received]) {
        //                 // TODO sending bytes to next nodes succeeded
        //             } else {
        //                 // TODO sending bytes to next nodes failed
        //             };
        //             ()
        //         }
        //         _ = collect_stats_interval.tick() => {
        //             // TODO collect/aggregate statistics each given time interval
        //             ()
        //         }
        //     }
        // }
    }
}
