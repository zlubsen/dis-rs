use dis_infra_core::runtime::default_runtime;

#[test]
fn runtime_with_config() {
    // let runtime = default_runtime().unwrap();
    // let spec = r#"
    //     [[ nodes ]]
    //     type = "udp"
    //     uri = "192.168.178.11:3000"
    //     interface = "192.168.178.11:3000"
    //     mode = "broadcast"
    //     ttl = 1
    //     block_own_socket = true
    //
    //     [[ nodes ]]
    //     type = "udp"
    //     uri = "192.168.178.11:4000"
    //     interface = "192.168.178.11:4000"
    //     mode = "unicast"
    //     ttl = 1
    //     block_own_socket = true
    // "#;
    // // let result = runtime.run_with_spec_string(spec);
    //
    // // assert!(result.is_ok());
}

#[test]
fn runtime_with_code() {
    let runtime = default_runtime().unwrap();

    runtime.run(async {});
}
