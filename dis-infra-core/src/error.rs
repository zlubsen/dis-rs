use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub(crate) enum InfraError {
    // ConfigFileLoad(io::Error),
    // ConfigFileRead(io::Error),
    // ConfigFileParse(toml::de::Error),
    // ConfigInvalid(ConfigError),
    RuntimeCannotStart(io::Error),
}

impl std::error::Error for InfraError {}

impl Display for InfraError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // GatewayError::ConfigFileLoad(err) => { write!(f, "Error loading config file - {}.", err) }
            // GatewayError::ConfigFileRead(err) => { write!(f, "Error reading config file contents - {}.", err) }
            // GatewayError::ConfigFileParse(err) => { write!(f, "Error parsing config file - {}.", err) }
            // GatewayError::ConfigInvalid(err) => { write!(f, "{}", err) }
            InfraError::RuntimeCannotStart(err) => { write!(f, "Error starting Tokio runtime: {err}") }
        }
    }
}