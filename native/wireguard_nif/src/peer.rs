//! nif bindings for wireguard peers

use std::convert::TryFrom;
use std::net::AddrParseError;
use std::time::SystemTime;

use rustler::{Error, NifResult, NifStruct};
use wireguard_control::{AllowedIp, PeerConfig, PeerConfigBuilder, PeerInfo, PeerStats};

use crate::key;

#[derive(NifStruct)]
#[module = "WireguardEx.PeerConfig"]
pub(crate) struct NifPeerConfig {
    public_key: String,
    preshared_key: Option<String>,
    endpoint: Option<String>,
    persistent_keepalive_interval: Option<u16>,
    allowed_ips: Vec<String>,
}

impl From<PeerConfig> for NifPeerConfig {
    fn from(config: PeerConfig) -> Self {
        Self {
            public_key: config.public_key.to_base64(),
            preshared_key: config.preshared_key.map(|k| k.to_base64()),
            endpoint: config.endpoint.map(|e| e.to_string()),
            persistent_keepalive_interval: config.persistent_keepalive_interval,
            allowed_ips: config
                .allowed_ips
                .iter()
                .map(|ip| format!("{}/{}", ip.address, ip.cidr))
                .collect(),
        }
    }
}

impl TryFrom<NifPeerConfig> for PeerConfigBuilder {
    type Error = Error;

    fn try_from(nif_config: NifPeerConfig) -> NifResult<Self> {
        let public_key = key::from_base64(&nif_config.public_key)?;
        let preshared_key = nif_config.preshared_key;
        let endpoint = nif_config.endpoint;
        let persistent_keepalive_interval = nif_config.persistent_keepalive_interval;
        let allowed_ips = nif_config
            .allowed_ips
            .iter()
            .map(|ip| {
                ip.parse().map_err(|_| {
                    Error::Term(Box::new(format!("Allowed ip failed to parse: {0}", ip)))
                })
            })
            .collect::<NifResult<Vec<AllowedIp>>>()?;

        let mut config = PeerConfigBuilder::new(&public_key);

        if let Some(preshared_key) = preshared_key {
            config = config.set_preshared_key(key::from_base64(&preshared_key)?);
        }
        if let Some(endpoint) = endpoint {
            config = config.set_endpoint(
                endpoint
                    .parse()
                    .map_err(|e: AddrParseError| Error::Term(Box::new(e.to_string())))?,
            );
        }
        if let Some(persistent_keepalive_interval) = persistent_keepalive_interval {
            config = config.set_persistent_keepalive_interval(persistent_keepalive_interval);
        }

        config = config.add_allowed_ips(&allowed_ips);

        Ok(config)
    }
}

#[derive(NifStruct)]
#[module = "WireguardEx.PeerInfo"]
pub(crate) struct NifPeerInfo {
    config: NifPeerConfig,
    stats: NifPeerStats,
}

impl From<PeerInfo> for NifPeerInfo {
    fn from(info: PeerInfo) -> Self {
        Self {
            config: info.config.into(),
            stats: info.stats.into(),
        }
    }
}

#[derive(NifStruct)]
#[module = "WireguardEx.PeerStats"]
struct NifPeerStats {
    last_handshake_time: Option<u64>,
    rx_bytes: u64,
    tx_bytes: u64,
}

impl From<PeerStats> for NifPeerStats {
    fn from(stats: PeerStats) -> Self {
        let last_handshake_time =
            stats
                .last_handshake_time
                .map(|t| match t.duration_since(SystemTime::UNIX_EPOCH) {
                    Ok(d) => d.as_secs(),
                    // This should be very very rare if it's even possible.
                    Err(_) => panic!("Last handshake time was before UNIX_EPOCH"),
                });

        Self {
            last_handshake_time,
            rx_bytes: stats.rx_bytes,
            tx_bytes: stats.tx_bytes,
        }
    }
}
