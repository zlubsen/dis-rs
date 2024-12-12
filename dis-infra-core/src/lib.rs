pub mod core;
pub mod runtime;
pub mod infra;
pub mod error;

#[cfg(test)]
mod tests {
    use crate::error::InfraError;
    // use crate::infra::UdpNode;
    use crate::runtime::default_runtime;

    #[tokio::test]
    fn basic_runtime_usage() {
    }
}