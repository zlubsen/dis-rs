use bytes::Bytes;
use gateway_core::error::GatewayError;
use gateway_core::runtime;
use gateway_core::runtime::{
    default_tokio_runtime, downcast_external_input, downcast_external_output, run_from_builder,
    Command, InfraBuilder,
};
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

    // TLDR; We can use the preset function to create all needed things in one go. Do provide the needed type hints.
    // The runtime provides a build-in event listener task, which trace!-es events.
    let (infra_runtime_builder, cmd_tx, _event_rx, input_tx, output_rx) =
        runtime::preset_builder_from_spec_str::<Bytes, Bytes>(spec).unwrap();
    let input_tx = input_tx.unwrap();
    let mut output_rx = output_rx.unwrap();

    // Alternatively, one can set up everything by hand.
    // Especially if the input/output channels and such are not needed, you can omit these parts.
    {
        // Initialise the InfraBuilder using `new()`
        let mut infra_runtime_builder = InfraBuilder::new();
        if let Err(err) = infra_runtime_builder.build_from_str(spec) {
            error!("{err}");
        }
        // We can now obtain handles to the coordination channels, for sending commands and receiving events
        let _cmd_tx = infra_runtime_builder.command_channel();
        let _event_rx = infra_runtime_builder.event_channel();

        // FIXME remove method or create default input/output spec definition
        // We can request a handle to an externalised input channel for a node, in this case 'Pass One'.
        // It needs to be downcast to the concrete type that you know yourself (depends on the node you created in the spec).
        // The Infra Runtime provides convenience functions to do this.
        let _input_tx =
            downcast_external_input::<Bytes>(infra_runtime_builder.external_input()).unwrap();
        // Similarly, we can request a handle to the externalised output channel, in this case 'Pass Two'.
        let _output_rx =
            downcast_external_output::<Bytes>(infra_runtime_builder.external_output()).unwrap();
    }

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
        // Spawn the actual nodes; This returns a JoinAll list with task handles, which can be awaited when needed.
        let join_all = run_from_builder(infra_runtime_builder).await?;
        // Thus await runtime completion, we send a Command::Quit concurrently after some time to stop in the cmd_handle.
        let _runtime_result = join_all.await;

        let _ = input_handle.await;
        let _ = output_handle.await;
        let _ = cmd_handle.await;

        info!("Done!");
        Ok::<(), GatewayError>(())
    }) {
        error!("{err}");
    }
}
