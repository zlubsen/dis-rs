pub mod core;
pub mod error;
pub mod infra;
pub mod runtime;

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn basic_runtime_usage() {}
}
