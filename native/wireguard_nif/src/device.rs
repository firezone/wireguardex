//! nif bindings for wireguard devices

use std::convert::{TryFrom, TryInto};

use rustler::{types::atom, Atom, Error, NifResult, NifStruct};
use wireguard_control::{
    Backend, Device, DeviceUpdate, InterfaceName, InvalidInterfaceName, PeerConfigBuilder,
};

use crate::key;
use crate::peer::{NifPeerConfig, NifPeerInfo};

#[cfg(target_os = "linux")]
const BACKEND: Backend = Backend::Kernel;
#[cfg(not(target_os = "linux"))]
const BACKEND: Backend = Backend::Userspace;

#[derive(NifStruct)]
#[module = "WireguardEx.Device"]
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

impl TryFrom<NifDeviceConfig> for DeviceUpdate {
    type Error = Error;

    fn try_from(config: NifDeviceConfig) -> NifResult<Self> {
        let mut device = DeviceUpdate::new();
        let public_key = config.public_key;
        let private_key = config.private_key;
        let fwmark = config.fwmark;
        let listen_port = config.listen_port;
        let peers = config
            .peers
            .into_iter()
            .map(|c| c.try_into())
            .collect::<NifResult<Vec<PeerConfigBuilder>>>()?;

        if let Some(public_key) = public_key {
            device = device.set_public_key(key::from_base64(&public_key)?);
        }
        if let Some(private_key) = private_key {
            device = device.set_private_key(key::from_base64(&private_key)?);
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

        Ok(device.add_peers(&peers))
    }
}

#[derive(NifStruct)]
#[module = "WireguardEx.DeviceConfig"]
struct NifDeviceConfig {
    public_key: Option<String>,
    private_key: Option<String>,
    fwmark: Option<u32>,
    listen_port: Option<u16>,
    peers: Vec<NifPeerConfig>,
    replace_peers: bool,
}

#[rustler::nif]
fn list_devices() -> NifResult<Vec<String>> {
    Ok(Device::list(BACKEND)
        .map_err(|e| Error::Term(Box::new(e.to_string())))?
        .iter()
        .map(|iname| iname.as_str_lossy().to_string())
        .collect())
}

#[rustler::nif]
fn get_device(name: &str) -> NifResult<NifDevice> {
    let iname = parse_iname(name)?;
    let device = Device::get(&iname, BACKEND).map_err(|e| Error::Term(Box::new(e.to_string())))?;

    Ok(device.into())
}

#[rustler::nif]
fn set_device(name: &str, config: NifDeviceConfig) -> NifResult<Atom> {
    let iname = parse_iname(name)?;
    let device: DeviceUpdate = config.try_into()?;

    device
        .apply(&iname, BACKEND)
        .map_err(|e| Error::Term(Box::new(e.to_string())))?;

    Ok(atom::ok())
}

#[rustler::nif]
fn delete_device(name: &str) -> NifResult<Atom> {
    let iname = parse_iname(name)?;
    let device = Device::get(&iname, BACKEND).map_err(|e| Error::Term(Box::new(e.to_string())))?;

    device
        .delete()
        .map_err(|e| Error::Term(Box::new(e.to_string())))?;

    Ok(atom::ok())
}

#[rustler::nif]
fn remove_peer(name: &str, public_key: &str) -> NifResult<Atom> {
    let iname = parse_iname(name)?;
    let key = key::from_base64(public_key)?;
    let device = DeviceUpdate::new().remove_peer_by_key(&key);

    device
        .apply(&iname, BACKEND)
        .map_err(|e| Error::Term(Box::new(e.to_string())))?;

    Ok(atom::ok())
}

fn parse_iname(name: &str) -> NifResult<InterfaceName> {
    name.parse()
        .map_err(|e: InvalidInterfaceName| Error::Term(Box::new(e.to_string())))
}
