use std::fmt::{Debug, Display, Formatter};
use std::net::SocketAddr;
use std::path::PathBuf;
use serde_derive::Deserialize;
use clap::Parser;
use cdis_assemble::codec::{CodecOptimizeMode, CodecUpdateMode, DEFAULT_HBT_CDIS_FULL_UPDATE_MPLIER};
use dis_rs::VariableParameters;

const DEFAULT_TTL: u16 = 1;
const DEFAULT_ENDPOINT_MODE: UdpMode = UdpMode::UniCast;
const DEFAULT_GATEWAY_MODE: CodecUpdateMode = CodecUpdateMode::FullUpdate;
const DEFAULT_SITE_HOST_PORT: u16 = 8080;
const DEFAULT_BLOCK_OWN_SOCKET: bool = true;
const DEFAULT_ENCODER_USE_GUISE: bool = false;
const DEFAULT_ENCODER_OPTIMIZATION: CodecOptimizeMode = CodecOptimizeMode::Completeness;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Config {
    pub(crate) dis_socket: UdpEndpoint,
    pub(crate) cdis_socket: UdpEndpoint,
    pub(crate) mode: GatewayMode,
    pub(crate) site_host: u16,
    pub(crate) federation_parameters: VariableParameters,
    pub(crate) hbt_cdis_full_update_mplier: f32,
    pub(crate) use_guise: bool,
    pub(crate) optimization: EncoderOptimization,
}

impl TryFrom<&ConfigSpec> for Config {
    type Error = ConfigError;

    fn try_from(value: &ConfigSpec) -> Result<Self, Self::Error> {
        let mode = if let Some(mode) = &value.update_mode {
            GatewayMode::try_from(mode.as_str())?
        } else {
            GatewayMode(DEFAULT_GATEWAY_MODE)
        };

        let dis_socket = UdpEndpoint::try_from(&value.dis)?;
        let cdis_socket = UdpEndpoint::try_from(&value.cdis)?;

        let site_host = (&value.site).map_or(DEFAULT_SITE_HOST_PORT, | spec | spec.port );

        let hbt_cdis_full_update_mplier = if let Some(fed_spec) = &value.federation {
            fed_spec.hbt_cdis_full_update_mplier.unwrap_or(DEFAULT_HBT_CDIS_FULL_UPDATE_MPLIER)
        } else { DEFAULT_HBT_CDIS_FULL_UPDATE_MPLIER };
        let federation_parameters = set_federation_parameters(&value.federation);

        let (use_guise, optimization) = if let Some(encoder) = &value.encoder {
            let use_guise = encoder.use_guise
                .map_or(DEFAULT_ENCODER_USE_GUISE, | val | val );
            let optimization = encoder.optimization.as_ref()
                .map_or(Ok(EncoderOptimization(DEFAULT_ENCODER_OPTIMIZATION)), |val| EncoderOptimization::try_from(val.as_str()) )?;
            (use_guise, optimization)
        } else { (DEFAULT_ENCODER_USE_GUISE, EncoderOptimization(DEFAULT_ENCODER_OPTIMIZATION)) };

        Ok(Self {
            dis_socket,
            cdis_socket,
            mode,
            site_host,
            federation_parameters,
            hbt_cdis_full_update_mplier,
            use_guise,
            optimization
        })
    }
}

/// Wrapper type for `CodecUpdateMode` from cdis_assemble crate
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub(crate) struct GatewayMode(pub(crate) CodecUpdateMode);

impl TryFrom<&str> for GatewayMode {
    type Error = ConfigError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "full_update" => Ok(GatewayMode(CodecUpdateMode::FullUpdate)),
            "partial_update" => Ok(GatewayMode(CodecUpdateMode::PartialUpdate)),
            _ => Err(ConfigError::GatewayModeInvalid)
        }

    }
}

impl Display for GatewayMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            CodecUpdateMode::FullUpdate => { write!(f, "Full Update") }
            CodecUpdateMode::PartialUpdate => { write!(f, "Partial Update") }
        }
    }
}

/// Wrapper type for `CodecOptimizeMode` from cdis_assemble crate
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub(crate) struct EncoderOptimization(pub(crate) CodecOptimizeMode);

impl TryFrom<&str> for EncoderOptimization {
    type Error = ConfigError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "bandwidth" => Ok(Self(CodecOptimizeMode::Bandwidth)),
            "completeness" => Ok(Self(CodecOptimizeMode::Completeness)),
            _ => Err(ConfigError::OptimizationModeInvalid)
        }
    }
}

impl Display for EncoderOptimization {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            CodecOptimizeMode::Bandwidth => { write!(f, "Bandwidth optimization") }
            CodecOptimizeMode::Completeness => { write!(f, "Completeness optimization") }
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

fn set_federation_parameters(spec: &Option<FederationSpec>) -> VariableParameters {
    let mut parameters = VariableParameters::default();
    if let Some(spec) = spec {
        if let Some(val) = spec.hbt_espdu_kind_cultural_feature {
            parameters.HBT_ESPDU_KIND_CULTURAL_FEATURE = val;
        }
        if let Some(val) = spec.hbt_espdu_kind_environmental {
            parameters.HBT_ESPDU_KIND_ENVIRONMENTAL = val;
        }
        if let Some(val) = spec.hbt_espdu_kind_expendable {
            parameters.HBT_ESPDU_KIND_EXPENDABLE = val;
        }
        if let Some(val) = spec.hbt_espdu_kind_life_form {
            parameters.HBT_ESPDU_KIND_LIFE_FORM = val;
        }
        if let Some(val) = spec.hbt_espdu_kind_munition {
            parameters.HBT_ESPDU_KIND_MUNITION = val;
        }
        if let Some(val) = spec.hbt_espdu_kind_radio {
            parameters.HBT_ESPDU_KIND_RADIO = val;
        }
        if let Some(val) = spec.hbt_espdu_kind_sensor {
            parameters.HBT_ESPDU_KIND_SENSOR = val;
        }
        if let Some(val) = spec.hbt_espdu_kind_emitter {
            parameters.HBT_ESPDU_KIND_EMITTER = val;
        }
        if let Some(val) = spec.hbt_espdu_kind_supply {
            parameters.HBT_ESPDU_KIND_SUPPLY = val;
        }
        if let Some(val) = spec.hbt_espdu_platform_air {
            parameters.HBT_ESPDU_PLATFORM_AIR = val;
        }
        if let Some(val) = spec.hbt_espdu_platform_land {
            parameters.HBT_ESPDU_PLATFORM_LAND = val;
        }
        if let Some(val) = spec.hbt_espdu_platform_space {
            parameters.HBT_ESPDU_PLATFORM_SPACE = val;
        }
        if let Some(val) = spec.hbt_espdu_platform_subsurface {
            parameters.HBT_ESPDU_PLATFORM_SUBSURFACE = val;
        }
        if let Some(val) = spec.hbt_espdu_platform_surface {
            parameters.HBT_ESPDU_PLATFORM_SURFACE = val;
        }
        if let Some(val) = spec.hbt_pdu_designator {
            parameters.HBT_PDU_DESIGNATOR = val;
        }
        if let Some(val) = spec.hbt_pdu_ee {
            parameters.HBT_PDU_EE = val;
        }
        if let Some(val) = spec.hbt_pdu_iff {
            parameters.HBT_PDU_IFF = val;
        }
        if let Some(val) = spec.hbt_pdu_transmitter {
            parameters.HBT_PDU_TRANSMITTER = val;
        }
        if let Some(val) = spec.hbt_stationary {
            parameters.HBT_STATIONARY = val;
        }
        if let Some(val) = spec.hbt_timeout_mplier {
            parameters.HBT_TIMEOUT_MPLIER = val;
        }
    }
    parameters
}

#[derive(Copy, Clone, Debug, Default)]
pub struct FederationAgreement {
    pub hbt_cdis_full_update_mplier: f32, //Default: 2.4
    pub hbt_espdu_kind_cultural_feature: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_environmental: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_expendable: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_life_form: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_munition: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_radio: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_sensor: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_emitter: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_supply: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_platform_air: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_platform_land: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_platform_space: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_platform_subsurface: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_platform_surface: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_pdu_designator: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_pdu_ee: f32, //Default: 5 s Tolerance: ±10%
    pub hbt_pdu_iff: f32, //Default: 10 s Tolerance: ±10%
    pub hbt_pdu_transmitter: f32, //Default: 2 s Tolerance: ±10%
    pub hbt_stationary: f32, //Default: 1 min Tolerance: ±10%
    pub hbt_timeout_mplier: f32, //Default: 2.4 (see NOTE 2)
}

impl TryFrom<&FederationSpec> for FederationAgreement {
    type Error = ConfigError;

    fn try_from(value: &FederationSpec) -> Result<Self, Self::Error> {
        Ok(Self {
            hbt_cdis_full_update_mplier: value.hbt_cdis_full_update_mplier.unwrap_or(DEFAULT_HBT_CDIS_FULL_UPDATE_MPLIER),
            hbt_espdu_kind_cultural_feature: value.hbt_espdu_kind_cultural_feature.unwrap_or(5.0),
            hbt_espdu_kind_environmental: value.hbt_espdu_kind_environmental.unwrap_or(5.0),
            hbt_espdu_kind_expendable: value.hbt_espdu_kind_expendable.unwrap_or(5.0),
            hbt_espdu_kind_life_form: value.hbt_espdu_kind_life_form.unwrap_or(5.0),
            hbt_espdu_kind_munition: value.hbt_espdu_kind_munition.unwrap_or(5.0),
            hbt_espdu_kind_radio: value.hbt_espdu_kind_radio.unwrap_or(5.0),
            hbt_espdu_kind_sensor: value.hbt_espdu_kind_sensor.unwrap_or(5.0),
            hbt_espdu_kind_emitter: value.hbt_espdu_kind_emitter.unwrap_or(5.0),
            hbt_espdu_kind_supply: value.hbt_espdu_kind_supply.unwrap_or(5.0),
            hbt_espdu_platform_air: value.hbt_espdu_platform_air.unwrap_or(5.0),
            hbt_espdu_platform_land: value.hbt_espdu_platform_land.unwrap_or(5.0),
            hbt_espdu_platform_space: value.hbt_espdu_platform_space.unwrap_or(5.0),
            hbt_espdu_platform_subsurface: value.hbt_espdu_platform_subsurface.unwrap_or(5.0),
            hbt_espdu_platform_surface: value.hbt_espdu_platform_surface.unwrap_or(5.0),
            hbt_pdu_designator: value.hbt_pdu_designator.unwrap_or(5.0),
            hbt_pdu_ee: value.hbt_pdu_ee.unwrap_or(5.0),
            hbt_pdu_iff: value.hbt_pdu_iff.unwrap_or(5.0),
            hbt_pdu_transmitter: value.hbt_pdu_transmitter.unwrap_or(5.0),
            hbt_stationary: value.hbt_stationary.unwrap_or(1.0),
            hbt_timeout_mplier: value.hbt_timeout_mplier.unwrap_or(2.4),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct ConfigSpec {
    pub metadata : Option<MetaData>,
    pub update_mode : Option<String>,
    pub block_host : Option<bool>,
    pub dis: EndPointSpec,
    pub cdis: EndPointSpec,
    pub site: Option<SiteSpec>,
    pub encoder: Option<EncoderSpec>,
    pub federation: Option<FederationSpec>,
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

#[derive(Debug, Clone, Deserialize)]
pub struct EncoderSpec {
    pub use_guise: Option<bool>,
    pub optimization: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FederationSpec {
    pub hbt_cdis_full_update_mplier: Option<f32>,
    pub hbt_espdu_kind_cultural_feature: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_environmental: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_expendable: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_life_form: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_munition: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_radio: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_sensor: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_emitter: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_kind_supply: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_platform_air: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_platform_land: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_platform_space: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_platform_subsurface: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_espdu_platform_surface: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_pdu_designator: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_pdu_ee: Option<f32>, //Default: 5 s Tolerance: ±10%
    pub hbt_pdu_iff: Option<f32>, //Default: 10 s Tolerance: ±10%
    pub hbt_pdu_transmitter: Option<f32>, //Default: 2 s Tolerance: ±10%
    pub hbt_stationary: Option<f32>, //Default: 1 min Tolerance: ±10%
    pub hbt_timeout_mplier: Option<f32>, //Default: 2.4 (see NOTE 2)
}

#[derive(Debug)]
pub enum ConfigError {
    GatewayModeInvalid,
    UdpModeInvalid,
    OptimizationModeInvalid,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::GatewayModeInvalid => { write!(f, "Configured Gateway mode is invalid. Valid values are 'full_update', and 'partial_update'.") }
            ConfigError::UdpModeInvalid => { write!(f, "Configured UDP mode is invalid. Valid values are 'unicast', 'broadcast' and 'multicast'.") }
            ConfigError::OptimizationModeInvalid => { write!(f, "Configured encoder Optimization mode is invalid. Valid values are 'bandwidth' and 'completeness'.") }
        }
    }
}

impl std::error::Error for ConfigError {}
