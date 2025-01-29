use dis_rs::entity_state::model::EntityState;
use dis_rs::enumerations::PduType;
use dis_rs::model::{Pdu, PduHeader, PduStatus};
use gateway_core::error::GatewayError;
use gateway_core::runtime;
use gateway_core::runtime::{default_tokio_runtime, run_from_builder, Command};
use std::time::Duration;
use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;
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

    let spec = nodes_specification();

    let runtime =
        default_tokio_runtime().expect("Expected tokio runtime to be created successfully.");

    let (infra_runtime_builder, cmd_tx, mut event_rx, input_tx, output_rx) =
        runtime::preset_builder_from_spec_str::<Pdu, Pdu>(spec).unwrap();
    let input_tx = input_tx.unwrap();
    let mut output_rx = output_rx.unwrap();

    if let Err(err) = runtime.block_on(async {
        let event_handle = tokio::spawn(async move {
            #[allow(clippy::while_let_loop)]
            loop {
                if let Ok(event) = event_rx.recv().await {
                    info!("{event:?}");
                } else {
                    break;
                }
            }
        });

        let input_handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(500)).await;

            let pdu = entity_state_pdu();

            info!("Sending a PDU to the DIS serialiser via the input channel.");
            let _ = input_tx.send(pdu.clone());
            (input_tx, pdu) // We return the sender handle to keep the channel open.
        });

        // Await and print the resulting output
        let output_handle = tokio::spawn(async move {
            info!("Waiting for a PDU to be delivered via the output channel.");
            let out = output_rx.recv().await.unwrap();
            info!("Received the PDU, initiating shutdown.");
            let _ = cmd_tx.send(Command::Quit);
            out
        });

        let join_all = run_from_builder(infra_runtime_builder).await?;
        // Await runtime completion, we send a Command::Quit after the message flow is complete.
        info!("Spawned all nodes, awaiting completion.");
        let _runtime_result = join_all.await;

        let (_input_tx, input_pdu) = input_handle.await.unwrap();
        let output_pdu = output_handle.await.unwrap();
        let _ = event_handle.await;

        // FIXME the header will be different due to the PduStatus field (None != Some(field::default, while both are 0 on the wire).
        // assert_eq!(input_pdu.header, output_pdu.header);
        assert_eq!(input_pdu.header.pdu_type, output_pdu.header.pdu_type);
        assert_eq!(input_pdu.header.pdu_length, output_pdu.header.pdu_length);

        info!("Done!");
        Ok::<(), GatewayError>(())
    }) {
        error!("{err}");
    }
}

/// The gateway specification creates three nodes: an UDP socket, a DIS parser/reader, and a DIS serialiser/writer.
/// Both DIS nodes connect to the UDP socket, to the outgoing and incoming channels respectively.
///
/// The UDP node allows to read back its own emitted data (via the 'block_own_socket = false' setting).
/// This way we can create a loop, where a PDU is serialised, broadcasted on the network, read back in and delivered to the application.
///
/// Whole chain: Input (DIS) > DIS serializer (Bytes) > UDP socket (Bytes) > DIS parser (DIS) > Output (DIS)
/// (Where besides the flow in this example the socket can also receive other packets from the network, and flow towards the output.)
fn nodes_specification() -> &'static str {
    r#"
        [[ nodes ]]
        type = "udp"
        name = "UDP socket"
        uri = "127.0.0.1:3000"
        interface = "127.0.0.1:3000"
        mode = "broadcast"
        ttl = 1
        block_own_socket = false

        [[ nodes ]]
        type = "dis_receiver"
        name = "DIS parser"
        exercise_id = 1
        allow_dis_versions = [6, 7]

        [[ nodes ]]
        type = "dis_sender"
        name = "DIS serialiser"
        exercise_id = 1
        allow_dis_versions = [6, 7]

        [[ channels ]]
        from = "UDP socket"
        to = "DIS parser"

        [[ channels ]]
        from = "DIS serialiser"
        to = "UDP socket"

        [ externals ]
        incoming = "DIS serialiser"
        outgoing = "DIS parser"
    "#
}

fn entity_state_pdu() -> Pdu {
    let header = PduHeader::new_v7(1, PduType::EntityState).with_pdu_status(PduStatus::default());
    let body = EntityState::builder()
        // here we can add more specific fields to the PduBody
        .build()
        .into_pdu_body();

    Pdu::finalize_from_parts(header, body, 100)
}
