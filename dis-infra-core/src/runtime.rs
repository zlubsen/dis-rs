use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use crate::core::Node;
use crate::error::InfraError;

pub struct InfraRuntime {
    async_runtime: Runtime,
    nodes: Vec<dyn Node>,
}

impl InfraRuntime {
    pub fn add_node(&mut self, node: impl Node) {
        self.nodes.push(node);
    }

    pub fn run(&self) {
        let handles = self.nodes.iter().map(|node| tokio::spawn(node.run())).map(|handle| handle.).collect::<Vec<JoinHandle<()>>>();
    }
}

/// Creates a default tokio runtime.
///
/// The runtime is multithreaded, enables IO and Time features, and enters the runtime context.
pub fn default_runtime() -> Result<InfraRuntime, InfraError> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build().map_err(|err| InfraError::RuntimeCannotStart(err))?;
    let _guard = runtime.enter();

    Ok(InfraRuntime {
        async_runtime: runtime,
        nodes: vec![],
    })
}