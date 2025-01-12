use dis_infra_core::runtime::{default_tokio_runtime, InfraRuntime};
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

    let spec = r#"
        [[ nodes ]]
        type = "udp"
        name = "UDP listener"
        uri = "127.0.0.1:3000"
        interface = "127.0.0.1:3001"
        mode = "broadcast"
        ttl = 1
        block_own_socket = true

        [[ nodes ]]
        type = "dis_receiver"
        name = "DIS parser"
        exercise_id = 1
        allow_dis_versions = [6, 7]

        [[ channels ]]
        from = "UDP listener"
        to = "DIS parser"
        "#;

    let mut runtime =
        default_tokio_runtime().expect("Expected tokio runtime to be created successfully.");
    let infra_runtime = InfraRuntime::init();

    if let Err(err) = runtime.block_on(infra_runtime.run_from_str(spec)) {
        println!("{err}");
    }
}
