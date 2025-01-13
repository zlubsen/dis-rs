use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

use dis_infra_core::runtime::{default_tokio_runtime, run_from_builder, InfraBuilder};

/// Demonstrates the basic use of the infrastructure
/// through the use of a specification (config) file.
///
/// Note that we call `runtime.run_with_spec_string(spec_as_&str)`,
/// while normally the spec would be in a normal `File`
/// Which one would call using `runtime.run_with_spec(path_to_file)`
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

    /// Create a specification in TOML format, either as a `&str` or a `Path` from the filesystem
    let spec = r#"
        [[ nodes ]]
        type = "udp"
        name = "UDP Socket"
        uri = "127.0.0.1:3000"
        interface = "127.0.0.1:3000"
        mode = "broadcast"
        ttl = 1
        block_own_socket = true

        [[ nodes ]]
        type = "pass_through"
        name = "Simple pass"

        [[ channels ]]
        from = "UDP Socket"
        to = "Simple pass"
    "#;

    /// Provide a (Tokio) runtime
    let runtime =
        default_tokio_runtime().expect("Expected tokio runtime to be created successfully.");

    /// Initialise the Infra
    let mut infra_runtime_builder = InfraBuilder::init();
    if let Err(err) = infra_runtime_builder.build_from_str(spec) {
        println!("{err}");
    }
    let cmd_tx = infra_runtime_builder.command_channel();
    let event_tx = infra_runtime_builder.event_channel();

    if let Err(err) = runtime.block_on(run_from_builder(infra_runtime_builder)) {
        println!("{err}");
    }
}
