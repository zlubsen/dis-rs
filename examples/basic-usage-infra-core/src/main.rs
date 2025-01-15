use bytes::Bytes;
use dis_infra_core::error::InfraError;
use dis_infra_core::runtime::{default_tokio_runtime, run_from_builder, Command, InfraBuilder};
use std::time::Duration;
use tokio::time::Instant;
use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

/// Demonstrates the basic use of the infrastructure
/// through the use of a specification (config) file.
///
/// Note that we call `runtime.build_from_str(spec_as_&str)`,
/// while normally the spec would be in a normal `File`
/// Which one would call using `runtime.build_from_path(path_to_file)`
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

    // Create a specification in TOML format, either as a `&str` or a `Path` from the filesystem
    //
    // This specification defines two nodes that simply pass on any data received, the first connects to the second.
    // The first node can receive data via an external incoming channel, the second node outputs via the external outgoing channel.
    // This way there is a complete chain from the external incoming channel to the external outgoing channel via the nodes.
    //
    // E.g.: incoming >> Pass One >> Pass Two >> outgoing
    let spec = r#"
        [[ nodes ]]
        type = "pass_through"
        name = "Pass One"

        [[ nodes ]]
        type = "pass_through"
        name = "Pass Two"

        [[ channels ]]
        from = "Pass One"
        to = "Pass Two"

        [ externals ]
        incoming = "Pass One"
        outgoing = "Pass Two"
    "#;

    // Provide a (Tokio) runtime
    let runtime =
        default_tokio_runtime().expect("Expected tokio runtime to be created successfully.");

    // Initialise the Infra
    let mut infra_runtime_builder = InfraBuilder::init();
    if let Err(err) = infra_runtime_builder.build_from_str(spec) {
        error!("{err}");
    }
    // We can now obtain handles to the coordination channels, for sending commands and receiving events
    let cmd_tx = infra_runtime_builder.command_channel();
    let _event_tx = infra_runtime_builder.event_channel();

    // We can request a handle to an externalised input channel for a node, in this case 'Pass One'.
    // It needs to be downcast to the concrete type that you know yourself (depends on the node you created in the spec).
    let input_tx = infra_runtime_builder
        .external_input()
        .map(|input| {
            *input
                .downcast::<tokio::sync::broadcast::Sender<Bytes>>()
                .unwrap()
        })
        .unwrap();
    // Similarly, we can request a handle to the externalised output channel, in this case 'Pass Two'.
    let mut output_rx = infra_runtime_builder
        .external_output()
        .map(|output| {
            output
                .downcast::<tokio::sync::broadcast::Receiver<Bytes>>()
                .unwrap()
        })
        .unwrap();

    const QUIT_DELAY: u64 = 4;
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
            let message = "Hello, World!";
            info!("Sending a message through the nodes: {}.", message);
            let _ = input_tx.send(Bytes::copy_from_slice(message.as_bytes()));
            input_tx // We return the sender handle to keep the channel open.
        });

        // Await and print the resulting output
        let output_handle = tokio::spawn(async move {
            let out = output_rx.recv().await.unwrap();
            info!("Received the message: {}", String::from_utf8_lossy(&out));
        });

        info!("Spawn the nodes");
        // Spawn the actual nodes, which is blocking
        run_from_builder(infra_runtime_builder).await?;

        let _ = input_handle.await;
        let _ = output_handle.await;
        let _ = cmd_handle.await;
        Ok::<(), InfraError>(())
    }) {
        error!("{err}");
    }
}
