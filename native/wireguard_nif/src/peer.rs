//! nif bindings for wireguard peers

use rustler::NifStruct;
// use std::time::{SystemTime, UNIX_EPOCH};
use wireguard_control::{AllowedIp, Key, PeerConfig, PeerConfigBuilder, PeerInfo, PeerStats};

#[derive(NifStruct)]
#[module = "Elixir.WireguardEx.PeerConfig"]
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
                .map(|ip| format!("{:?}", ip))
                .collect(),
        }
    }
}

impl From<NifPeerConfig> for PeerConfigBuilder {
    fn from(nif_config: NifPeerConfig) -> Self {
        let public_key = Key::from_base64(&nif_config.public_key).unwrap();
        let preshared_key = nif_config.preshared_key;
        let endpoint = nif_config.endpoint;
        let persistent_keepalive_interval = nif_config.persistent_keepalive_interval;
        let allowed_ips = nif_config
            .allowed_ips
            .iter()
            .map(|ip| ip.parse().unwrap())
            .collect::<Vec<AllowedIp>>();

        let mut config = PeerConfigBuilder::new(&public_key);

        if let Some(preshared_key) = preshared_key {
            config = config.set_preshared_key(Key::from_base64(&preshared_key).unwrap());
        }
        if let Some(endpoint) = endpoint {
            config = config.set_endpoint(endpoint.parse().unwrap());
        }
        if let Some(persistent_keepalive_interval) = persistent_keepalive_interval {
            config = config.set_persistent_keepalive_interval(persistent_keepalive_interval);
        }

        config = config.add_allowed_ips(&allowed_ips);

        config
    }
}

#[derive(NifStruct)]
#[module = "Elixir.WireguardEx.PeerInfo"]
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
#[module = "Elixir.WireguardEx.PeerStats"]
struct NifPeerStats {
    last_handshake_time: Option<u64>,
    rx_bytes: u64,
    tx_bytes: u64,
}

impl From<PeerStats> for NifPeerStats {
    fn from(stats: PeerStats) -> Self {
        Self {
            last_handshake_time: stats.last_handshake_time.map(|_t| {
                // TODO maybe convert the SystemTime object using UNIX_EPOCH to a u64
                0
            }),
            rx_bytes: stats.rx_bytes,
            tx_bytes: stats.tx_bytes,
        }
    }
}
