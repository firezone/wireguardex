//! nif bindings for wireguard peers

use rustler::NifStruct;
use std::time::{SystemTime, UNIX_EPOCH};
use wireguard_control::{PeerConfig, PeerConfigBuilder, PeerInfo, PeerStats, Key};

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
        let mut config = PeerConfigBuilder::new(&public_key);

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
                // TODO handshake time set via linux kernel fn ktime_get_real_ts64
                // maybe convert the SystemTime object using UNIX_EPOCH to a u64
                0
            }),
            rx_bytes: stats.rx_bytes,
            tx_bytes: stats.tx_bytes,
        }
    }
}
