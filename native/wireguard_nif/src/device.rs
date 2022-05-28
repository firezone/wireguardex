//! nif bindings for wireguard devices

use rustler::{Error, NifResult, NifStruct};
use wireguard_control::{Backend, Device, DeviceUpdate, InterfaceName, Key, PeerConfigBuilder};

use crate::key;
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

impl From<NifDeviceConfig> for DeviceUpdate {
    fn from(config: NifDeviceConfig) -> Self {
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

        device.add_peers(&peers)
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
fn get_device(name: &str) -> NifResult<NifDevice> {
    let iname = parse_iname(name)?;
    let device = Device::get(&iname, BACKEND).unwrap();

    Ok(device.into())
}

#[rustler::nif]
fn set_device(name: &str, config: NifDeviceConfig) -> NifResult<()> {
    let iname = parse_iname(name)?;
    let device: DeviceUpdate = config.into();

    device.apply(&iname, BACKEND).unwrap();

    Ok(())
}

#[rustler::nif]
fn delete_device(name: &str) -> NifResult<()> {
    let iname = parse_iname(name)?;
    let device = Device::get(&iname, BACKEND).unwrap();

    device.delete().unwrap();

    Ok(())
}

#[rustler::nif]
fn remove_peer(name: &str, public_key: &str) -> NifResult<()> {
    let iname = parse_iname(name)?;
    let key = key::from_base64(public_key)?;
    let device = DeviceUpdate::new().remove_peer_by_key(&key);

    device.apply(&iname, BACKEND).unwrap();

    Ok(())
}

fn parse_iname(name: &str) -> NifResult<InterfaceName> {
    // Parse an interface name string into a valid InterfaceName struct.
    // Log an error with the name if it fails and map to a rustler error.
    name.parse().map_err(|_e| {
        eprintln!("[Error] Invalid interface name: {0}", name);
        Error::BadArg
    })
}
