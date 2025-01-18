use bytes::Bytes;
use gateway_core::runtime::{
    downcast_external_input, downcast_external_output, run_from_builder, Command, InfraBuilder,
};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::test(flavor = "multi_thread")]
async fn udp() {}

#[tokio::test(flavor = "multi_thread")]
async fn tcp_server() {
    let spec = r#"
        [[ nodes ]]
        type = "tcp_server"
        name = "TCP Server"
        interface = "127.0.0.1:3002"
        max_connections = 1

        [[ nodes ]]
        type = "pass_through"
        name = "PassThrough"

        [[ channels ]]
        from = "TCP Server"
        to = "PassThrough"

        [ externals ]
        incoming = "TCP Server"
        outgoing = "PassThrough"
    "#;

    const DELAY: u64 = 500;
    const SERVER_ADDRESS: &str = "127.0.0.1:3002";

    let mut infra_runtime_builder = InfraBuilder::init();
    if let Err(err) = infra_runtime_builder.build_from_str(spec) {
        assert!(false, "{err}");
    }

    let cmd_tx = infra_runtime_builder.command_channel();
    let _event_tx = infra_runtime_builder.event_channel();
    let input_tx =
        downcast_external_input::<Bytes>(infra_runtime_builder.external_input()).unwrap();
    let mut output_tx =
        downcast_external_output::<Bytes>(infra_runtime_builder.external_output()).unwrap();

    let stimulus_handle = tokio::spawn(async move {
        let message = "Hello, World!";
        let mut receive_buffer: [u8; 13] = [0; 13];

        tokio::time::sleep(Duration::from_millis(DELAY)).await; // wait for runtime to be started.

        let res = TcpStream::connect(SERVER_ADDRESS).await;
        let (mut tcp_rx, mut tcp_tx) = res.unwrap().into_split();

        let _ = input_tx.send(Bytes::copy_from_slice(message.as_bytes()));
        if tcp_rx.readable().await.is_ok() {
            let tcp_out_received = tcp_rx.read(&mut receive_buffer).await;
            assert!(tcp_out_received.is_ok(), "TCP returned by node is not Ok.");
            assert_eq!(
                tcp_out_received.unwrap(),
                message.as_bytes().len(),
                "Message returned over TCP socket by node is not equal to the input."
            );
        } else {
            assert!(false, "Failed to read from the TCP connection.");
        }

        let _ = tcp_tx.write(message.as_bytes()).await;
        let node_out_received = output_tx.recv().await;

        assert!(node_out_received.is_ok());
        let node_out_received = node_out_received.unwrap();
        assert_eq!(
            (&node_out_received).len(),
            message.as_bytes().len(),
            "Length of the returned data is not equal to the input."
        );
        assert_eq!(
            String::from_utf8_lossy(&node_out_received),
            message,
            "Message output is not equal to the input."
        );

        let _ = cmd_tx.send(Command::Quit);
    });

    let runner_handles = run_from_builder(infra_runtime_builder).await.unwrap().await;

    runner_handles
        .iter()
        .for_each(|handle| assert!(handle.is_ok(), "Runtime not stopped correctly"));

    let _ = stimulus_handle.await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn tcp_client() {
    let spec = r#"
        [[ nodes ]]
        type = "tcp_client"
        name = "TCP Client"
        interface = "127.0.0.1:2000"
        address = "127.0.0.1:3003"

        [[ nodes ]]
        type = "pass_through"
        name = "PassThrough"

        [[ channels ]]
        from = "TCP Client"
        to = "PassThrough"

        [ externals ]
        incoming = "TCP Client"
        outgoing = "PassThrough"
    "#;

    const DELAY: u64 = 500;
    const SERVER_ADDRESS: &str = "127.0.0.1:3003";

    let mut infra_runtime_builder = InfraBuilder::init();
    if let Err(err) = infra_runtime_builder.build_from_str(spec) {
        assert!(false, "{err}");
    }

    let cmd_tx = infra_runtime_builder.command_channel();
    let _event_tx = infra_runtime_builder.event_channel();

    let input_tx =
        downcast_external_input::<Bytes>(infra_runtime_builder.external_input()).unwrap();
    let mut output_tx =
        downcast_external_output::<Bytes>(infra_runtime_builder.external_output()).unwrap();

    let stimulus_handle = tokio::spawn(async move {
        let message = "Hello, World!";
        let mut receive_buffer: [u8; 13] = [0; 13];

        let listener = TcpListener::bind(SERVER_ADDRESS).await.unwrap();
        let (stream, _address) = listener.accept().await.unwrap();
        let (mut tcp_rx, mut tcp_tx) = stream.into_split();

        let _ = input_tx.send(Bytes::copy_from_slice(message.as_bytes()));
        if tcp_rx.readable().await.is_ok() {
            let tcp_out_received = tcp_rx.read(&mut receive_buffer).await;
            assert!(tcp_out_received.is_ok(), "TCP returned by node is not Ok.");
            assert_eq!(
                tcp_out_received.unwrap(),
                message.as_bytes().len(),
                "Message returned over TCP socket by node is not equal to the input."
            );
        } else {
            assert!(false, "Failed to read from the TCP connection.");
        }

        let _ = tcp_tx.write(message.as_bytes()).await;
        let node_out_received = output_tx.recv().await;

        assert!(node_out_received.is_ok());
        let node_out_received = node_out_received.unwrap();
        assert_eq!(
            (&node_out_received).len(),
            message.as_bytes().len(),
            "Length of the returned data is not equal to the input."
        );
        assert_eq!(
            String::from_utf8_lossy(&node_out_received),
            message,
            "Message output is not equal to the input."
        );

        let _ = cmd_tx.send(Command::Quit);
    });

    tokio::time::sleep(Duration::from_millis(DELAY)).await; // give stimulus time to setup the TCP Server
    let runner_handles = run_from_builder(infra_runtime_builder).await.unwrap().await;

    let _ = stimulus_handle.await.unwrap();

    runner_handles
        .iter()
        .for_each(|handle| assert!(handle.is_ok(), "Runtime not stopped correctly"));
}
