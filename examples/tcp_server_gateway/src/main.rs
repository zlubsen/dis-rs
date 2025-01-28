use bytes::Bytes;
use gateway_core::runtime::{
    downcast_external_input, downcast_external_output, run_from_builder, Command, InfraBuilder,
};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::trace;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

/// Demonstrates the basic use of the infrastructure
/// through the use of a specification (config) file.
///
/// Note that we call `runtime.build_from_str(spec_as_&str)`,
/// while normally the spec would be in a normal `File`
/// Which one would call using `runtime.build_from_path(path_to_file)`
#[tokio::main]
async fn main() {
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

    // let runtime =
    //     default_tokio_runtime().expect("Expected tokio runtime to be created successfully.");

    const DELAY: u64 = 500;
    const SERVER_ADDRESS: &str = "127.0.0.1:3030";

    let mut infra_runtime_builder = InfraBuilder::new();
    if let Err(err) = infra_runtime_builder.build_from_str(spec) {
        assert!(false, "{err}");
    } else {
        trace!("Gateway build from spec Ok");
    }

    let cmd_tx = infra_runtime_builder.command_channel();
    let _event_tx = infra_runtime_builder.event_channel();
    let input_tx =
        downcast_external_input::<Bytes>(infra_runtime_builder.external_input()).unwrap();
    let mut output_rx =
        downcast_external_output::<Bytes>(infra_runtime_builder.external_output()).unwrap();

    let stimulus_handle = tokio::spawn(async move {
        trace!("Stimulus: Spawning task");
        let message = "Hello, World!";
        let mut receive_buffer: [u8; 13] = [0; 13];

        tokio::time::sleep(Duration::from_millis(DELAY)).await; // wait for runtime to be started.

        trace!("Stimulus: Connect to the TCP Server socket");
        let res = TcpStream::connect(SERVER_ADDRESS).await;
        let (mut tcp_rx, mut tcp_tx) = res.unwrap().into_split();

        trace!("Stimulus: sending a message to the input channel");
        let _ = input_tx.send(Bytes::copy_from_slice(message.as_bytes()));

        let tcp_out_received = tcp_rx.read(&mut receive_buffer).await;
        assert!(tcp_out_received.is_ok(), "TCP returned by node is not Ok.");
        assert_eq!(
            tcp_out_received.unwrap(),
            message.as_bytes().len(),
            "Message returned over TCP socket by node is not equal in length to the input."
        );

        trace!("Stimulus: sending a message to the TCP Server");
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

        trace!("Stimulus: Sending Command::Quit");
        let _ = cmd_tx.send(Command::Quit);
    });

    trace!("Spawning all node tasks");
    let runner_handles = run_from_builder(infra_runtime_builder).await.unwrap().await;

    trace!("Awaiting nodes completion");
    runner_handles
        .iter()
        .for_each(|handle| assert!(handle.is_ok(), "Runtime not stopped correctly"));

    trace!("Awaiting stimulus completion");
    let res = stimulus_handle.await;
    assert!(res.is_ok());
    res.unwrap();
}
