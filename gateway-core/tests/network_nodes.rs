use bytes::Bytes;
use gateway_core::runtime::{
    downcast_external_input, downcast_external_output, run_from_builder, Command, InfraBuilder,
};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};

#[tokio::test(flavor = "multi_thread")]
async fn udp() {
    let spec = r#"
        [[ nodes ]]
        type = "udp"
        name = "UDP"
        uri = "127.0.0.1:6666"
        interface = "127.0.0.1:5555"
        mode = "broadcast"
        ttl = 1
        block_own_socket = true

        [[ nodes ]]
        type = "pass_through"
        name = "PassThrough"

        [[ channels ]]
        from = "UDP"
        to = "PassThrough"

        [ externals ]
        incoming = "UDP"
        outgoing = "PassThrough"
    "#;

    const DELAY: u64 = 500;
    const NODE_ADDRESS: &str = "127.0.0.1:5555";
    const REMOTE_SOCKET_ADDRESS: &str = "127.0.0.1:6666";

    let mut infra_runtime_builder = InfraBuilder::new();
    if let Err(err) = infra_runtime_builder.build_from_str(spec) {
        assert!(false, "{err}");
    }

    let cmd_tx = infra_runtime_builder.command_channel();
    let _event_tx = infra_runtime_builder.event_channel();
    let input_tx =
        downcast_external_input::<Bytes>(infra_runtime_builder.external_input()).unwrap();
    let mut output_rx =
        downcast_external_output::<Bytes>(infra_runtime_builder.external_output()).unwrap();

    let stimulus_handle = tokio::spawn(async move {
        let message = "Hello, World!";
        let mut receive_buffer: [u8; 13] = [0; 13];

        tokio::time::sleep(Duration::from_millis(DELAY)).await; // wait for runtime to be started.

        let sock = UdpSocket::bind(REMOTE_SOCKET_ADDRESS.parse::<SocketAddr>().unwrap())
            .await
            .unwrap();
        let node_addr = NODE_ADDRESS.parse::<SocketAddr>().unwrap();
        sock.connect(node_addr).await.unwrap();

        // input via incoming channel goes out via the socket
        let _ = input_tx.send(Bytes::copy_from_slice(message.as_bytes()));
        let _bytes_send = sock.recv(&mut receive_buffer).await.unwrap();

        // input through the socket comes out via the outgoing channel
        sock.send(message.as_bytes()).await.unwrap();
        let node_out_received = output_rx.recv().await;

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

    let res = stimulus_handle.await;
    assert!(res.is_ok());
    res.unwrap();
}

#[ignore]
#[tokio::test(flavor = "multi_thread")]
async fn tcp_server() {
    let spec = r#"
        [[ nodes ]]
        type = "tcp_server"
        name = "TCP Server"
        interface = "127.0.0.1:3030"
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
    const SERVER_ADDRESS: &str = "127.0.0.1:3030";

    let mut infra_runtime_builder = InfraBuilder::new();
    if let Err(err) = infra_runtime_builder.build_from_str(spec) {
        assert!(false, "{err}");
    }

    let cmd_tx = infra_runtime_builder.command_channel();
    let _event_tx = infra_runtime_builder.event_channel();
    let input_tx =
        downcast_external_input::<Bytes>(infra_runtime_builder.external_input()).unwrap();
    let mut output_rx =
        downcast_external_output::<Bytes>(infra_runtime_builder.external_output()).unwrap();

    let stimulus_handle = tokio::spawn(async move {
        let message = "Hello, World!";
        let mut receive_buffer: [u8; 13] = [0; 13];

        tokio::time::sleep(Duration::from_millis(DELAY)).await; // wait for runtime to be started.

        let res = TcpStream::connect(SERVER_ADDRESS).await;
        let (tcp_rx, mut tcp_tx) = res.unwrap().into_split();

        let _ = input_tx.send(Bytes::copy_from_slice(message.as_bytes()));

        let tcp_out_received = match tcp_rx.readable().await {
            Ok(_) => tcp_rx.try_read(&mut receive_buffer),
            Err(err) => Err(err),
        };

        match &tcp_out_received {
            Ok(0) => {
                assert!(false, "The TCP reader half is closed");
            }
            Ok(bytes_received) => {
                assert_eq!(
                    *bytes_received,
                    message.as_bytes().len(),
                    "Message returned over TCP socket by node is not equal in length to the input."
                );
            }
            Err(err) => {
                assert!(false, "{err}")
            }
        }

        let _ = tcp_tx.write(message.as_bytes()).await;
        let node_out_received = output_rx.recv().await;

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

    let res = stimulus_handle.await;
    assert!(res.is_ok());
    res.unwrap();
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

    let mut infra_runtime_builder = InfraBuilder::new();
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

    runner_handles
        .iter()
        .for_each(|handle| assert!(handle.is_ok(), "Runtime not stopped correctly"));

    let res = stimulus_handle.await;
    assert!(res.is_ok());
    res.unwrap();
}
