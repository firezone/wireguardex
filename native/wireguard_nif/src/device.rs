//! nif bindings for wireguard devices

use rustler::NifStruct;
use wireguard_control::{Backend, Device, DeviceUpdate, Key, PeerConfigBuilder};

use crate::peer::{NifPeerConfig, NifPeerInfo};

#[cfg(target_os = "linux")]
const BACKEND: Backend = Backend::Kernel;
#[cfg(not(target_os = "linux"))]
const BACKEND: Backend = Backend::Userspace;

#[derive(NifStruct)]
#[module = "Elixir.WireguardEx.Device"]
struct NifDevice {
    name: String,
    public_key: Option<String>,
    private_key: Option<String>,
    fwmark: Option<u32>,
    listen_port: Option<u16>,
    peers: Vec<NifPeerInfo>,
    linked_name: Option<String>,
}

impl From<Device> for NifDevice {
    fn from(d: Device) -> Self {
        Self {
            name: d.name.as_str_lossy().to_string(),
            public_key: d.public_key.map(|k| k.to_base64()),
            private_key: d.private_key.map(|k| k.to_base64()),
            fwmark: d.fwmark,
            listen_port: d.listen_port,
            peers: d.peers.into_iter().map(|p| p.into()).collect(),
            linked_name: d.linked_name,
        }
    }
}

#[derive(NifStruct)]
#[module = "Elixir.WireguardEx.DeviceConfig"]
struct NifDeviceConfig {
    public_key: Option<String>,
    private_key: Option<String>,
    fwmark: Option<u32>,
    listen_port: Option<u16>,
    peers: Vec<NifPeerConfig>,
    replace_peers: bool,
}

#[rustler::nif]
fn list_devices() -> Vec<String> {
    Device::list(BACKEND)
        .unwrap()
        .iter()
        .map(|iname| iname.as_str_lossy().to_string())
        .collect()
}

#[rustler::nif]
fn get_device(name: &str) -> NifDevice {
    let device = Device::get(&name.parse().unwrap(), BACKEND).unwrap();

    device.into()
}

#[rustler::nif]
fn set_device(name: &str, config: NifDeviceConfig) {
    let mut device = DeviceUpdate::new();
    let public_key = config.public_key;
    let private_key = config.private_key;
    let fwmark = config.fwmark;
    let listen_port = config.listen_port;
    let peers = config
        .peers
        .into_iter()
        .map(|c| c.into())
        .collect::<Vec<PeerConfigBuilder>>();

    if let Some(public_key) = public_key {
        device = device.set_public_key(Key::from_base64(&public_key).unwrap());
    }
    if let Some(private_key) = private_key {
        device = device.set_private_key(Key::from_base64(&private_key).unwrap());
    }
    if let Some(fwmark) = fwmark {
        device = device.set_fwmark(fwmark);
    }
    if let Some(listen_port) = listen_port {
        device = device.set_listen_port(listen_port);
    }
    if config.replace_peers {
        device = device.replace_peers();
    }

    device = device.add_peers(&peers);

    device.apply(&name.parse().unwrap(), BACKEND).unwrap();
}

#[rustler::nif]
fn delete_device(name: &str) {
    let device = Device::get(&name.parse().unwrap(), BACKEND).unwrap();

    device.delete().unwrap();
}
