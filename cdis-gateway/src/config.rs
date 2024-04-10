use std::net::{IpAddr, SocketAddr};

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct Config {
    pub(crate) dis_socket: UdpEndpoint,
    pub(crate) cdis_socket: UdpEndpoint,
    pub(crate) mode: GatewayMode,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct UdpEndpoint {
    pub(crate) mode: UdpMode,
    pub(crate) interface: IpAddr,
    pub(crate) address: SocketAddr,
    pub(crate) ttl: u16,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub(crate) enum UdpMode {
    #[default]
    UniCast,
    BroadCast,
    MultiCast,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub(crate) enum GatewayMode {
    #[default]
    FullUpdate,
    PartialUpdate,
}