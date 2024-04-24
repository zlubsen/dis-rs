use std::fmt::{Debug, Display, Formatter};
use std::net::{SocketAddr, ToSocketAddrs};
use std::path::PathBuf;
use serde_derive::Deserialize;
use clap::Parser;

const DEFAULT_TTL: u16 = 1;
const DEFAULT_ENDPOINT_MODE: UdpMode = UdpMode::UniCast;
const DEFAULT_GATEWAY_MODE: GatewayMode = GatewayMode::FullUpdate;
const DEFAULT_SITE_HOST_PORT: u16 = 8080;
const DEFAULT_BLOCK_OWN_SOCKET: bool = true;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct Config {
    pub(crate) dis_socket: UdpEndpoint,
    pub(crate) cdis_socket: UdpEndpoint,
    pub(crate) mode: GatewayMode,
    pub(crate) site_host: u16,
}

impl TryFrom<&ConfigSpec> for Config {
    type Error = ConfigError;

    fn try_from(value: &ConfigSpec) -> Result<Self, Self::Error> {
        let mode = if let Some(mode) = &value.mode {
            GatewayMode::try_from(mode.as_str())?
        } else {
            DEFAULT_GATEWAY_MODE
        };

        let dis_socket = UdpEndpoint::try_from(&value.dis)?;
        let cdis_socket = UdpEndpoint::try_from(&value.cdis)?;

        let site_host = (&value.site).map_or(DEFAULT_SITE_HOST_PORT, | spec | spec.port );

        Ok(Self {
            dis_socket,
            cdis_socket,
            mode,
            site_host,
        })
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub(crate) enum GatewayMode {
    #[default]
    FullUpdate,
    PartialUpdate,
}

impl TryFrom<&str> for GatewayMode {
    type Error = ConfigError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "fullupdate" => Ok(Self::FullUpdate),
            "partialupdate" => Ok(Self::PartialUpdate),
            _ => Err(ConfigError::GatewayModeInvalid)
        }

    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct UdpEndpoint {
    pub(crate) mode: UdpMode,
    pub(crate) interface: SocketAddr,
    pub(crate) address: SocketAddr,
    pub(crate) ttl: u16,
    pub(crate) block_own_socket: bool,
}

impl TryFrom<&EndPointSpec> for UdpEndpoint {
    type Error = ConfigError;

    fn try_from(value: &EndPointSpec) -> Result<Self, Self::Error> {
        let mode = if let Some(mode) = &value.mode {
            UdpMode::try_from(mode.as_str())?
        } else {
            DEFAULT_ENDPOINT_MODE
        };
        let ttl = value.ttl.unwrap_or(DEFAULT_TTL);
        let block_own_socket = value.block_own_socket.unwrap_or(DEFAULT_BLOCK_OWN_SOCKET);

        let interface = value.interface.parse().expect(format!("Cannot parse socket address {}", value.interface).as_str());
        let address = value.interface.parse().expect(format!("Cannot parse socket address {}", value.interface).as_str());

        Ok(Self {
            mode,
            interface,
            address,
            ttl,
            block_own_socket,
        })
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub(crate) enum UdpMode {
    #[default]
    UniCast,
    BroadCast,
    MultiCast,
}

impl TryFrom<&str> for UdpMode {
    type Error = ConfigError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "unicast" => Ok(Self::UniCast),
            "broadcast" => Ok(Self::BroadCast),
            "multicast" => Ok(Self::MultiCast),
            _ => Err(ConfigError::UdpModeInvalid)
        }

    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// Sets the config file, positional argument
    #[arg(value_name = "FILE")]
    pub config: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct ConfigSpec {
    pub metadata : Option<MetaData>,
    pub mode : Option<String>,
    pub block_host : Option<bool>,
    pub dis: EndPointSpec,
    pub cdis: EndPointSpec,
    pub site: Option<SiteSpec>,
}

#[derive(Debug, Deserialize)]
pub struct MetaData {
    pub name : String,
    pub author : String,
    pub version : String,
}

#[derive(Debug, Deserialize)]
pub struct EndPointSpec {
    pub uri: String,
    pub interface: String,
    pub mode: Option<String>,
    pub ttl: Option<u16>,
    pub block_own_socket: Option<bool>,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct SiteSpec {
    pub port: u16,
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    GatewayModeInvalid,
    UdpModeInvalid,
    CannotParseSocketAddress,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound => { write!(f, "File not found.") }
            ConfigError::GatewayModeInvalid => { write!(f, "Configured Gateway mode is invalid. Valid values are 'fullupdate', and 'partialupdate'.") }
            ConfigError::UdpModeInvalid => { write!(f, "Configured UDP mode is invalid. Valid values are 'unicast', 'broadcast' and 'multicast'.") }
            ConfigError::CannotParseSocketAddress => { write!(f, "Could not parse the provided socket address.") }
        }
    }
}

impl std::error::Error for ConfigError {}
