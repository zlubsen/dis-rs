pub mod core;
pub mod runtime;
pub mod infra;
pub mod error;

#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;
    use crate::error::InfraError;
    use crate::infra::UdpNode;
    use crate::runtime::default_runtime;

    #[test]
    fn basic_runtime_usage() {
        let mut runtime = match default_runtime() {
            Ok(rt) => { rt }
            Err(_) => { assert!(false); return }
        };

        let udp_node: UdpNode = UdpNode::new_broadcast_socket(1);
        runtime.add_node(udp_node);

        runtime.run();
    }
}