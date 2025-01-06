use dis_infra_core::runtime::default_runtime;

/// Demonstrates the basic use of the infrastructure
/// through the use of a specification (config) file.
///
/// Note that we call `runtime.run_with_spec_string(spec_as_&str)`,
/// while normally the spec would be in a normal `File`
/// Which one would call using `runtime.run_with_spec(path_to_file)`
fn main() {
    let spec = r#"
        [[ nodes ]]
        type = "udp"
        name = "UDP 1"
        uri = "192.168.178.11:3000"
        interface = "192.168.178.11:3000"
        mode = "broadcast"
        ttl = 1
        block_own_socket = true

        [[ nodes ]]
        type = "udp"
        name = "UDP 2"
        uri = "192.168.178.11:4000"
        interface = "192.168.178.11:4000"
        mode = "unicast"
        ttl = 1
        block_own_socket = true

        [[ channels ]]
        from = "UDP 1"
        to = "UDP 2"
    "#;

    let mut runtime = default_runtime().unwrap();

    if let Err(err) = runtime.run_with_spec_string(spec) {
        println!("{err}");
    }
}
