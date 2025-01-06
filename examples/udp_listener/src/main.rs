#[tokio::main]
async fn main() {
    //// TODO This is old stuff; convert to smaller tests

    // let (command_tx, command_rx) = tokio::sync::broadcast::channel(10);
    // let (event_tx, event_rx) = tokio::sync::mpsc::channel(10);
    //
    // let udp_node_spec = r#"
    //     type = "udp"
    //     uri = "192.168.178.11:4001"
    //     interface = "192.168.178.11:4001"
    //     mode = "broadcast"
    //     ttl = 1
    //     block_own_socket = true
    //     "#;
    // dbg!(udp_node_spec);
    //
    // let config = udp_node_spec.parse::<Table>().unwrap();
    //
    // let node = node_data_from_spec(1, command_rx, event_tx, &config).unwrap();
    //
    // let handle = node.spawn_into_runner();
    // handle.await.expect("Error awaiting node JoinHandle");
}
