//! nif bindings for wireguard devices

use std::convert::{TryFrom, TryInto};

use rustler::{types::atom, Atom, Error, NifResult, NifStruct};
use wireguard_control::{Backend, Device, DeviceUpdate, InterfaceName, PeerConfigBuilder};

use crate::key::{vec_to_key, NifKey};
use crate::peer::{NifPeerConfig, NifPeerInfo};

#[cfg(target_os = "linux")]
const BACKEND: Backend = Backend::Kernel;
#[cfg(not(target_os = "linux"))]
const BACKEND: Backend = Backend::Userspace;

#[derive(NifStruct)]
#[module = "Wireguardex.Device"]
struct NifDevice {
    name: String,
    public_key: Option<NifKey>,
    private_key: Option<NifKey>,
    fwmark: Option<u32>,
    listen_port: Option<u16>,
    peers: Vec<NifPeerInfo>,
    linked_name: Option<String>,
}

impl From<Device> for NifDevice {
    fn from(d: Device) -> Self {
        Self {
            name: d.name.as_str_lossy().to_string(),
            public_key: d.public_key.map(|k| k.0.to_vec()),
            private_key: d.private_key.map(|k| k.0.to_vec()),
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
            device = device.set_public_key(vec_to_key(public_key)?);
        }
        if let Some(private_key) = private_key {
            device = device.set_private_key(vec_to_key(private_key)?);
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
#[module = "Wireguardex.DeviceConfig"]
struct NifDeviceConfig {
    public_key: Option<NifKey>,
    private_key: Option<NifKey>,
    fwmark: Option<u32>,
    listen_port: Option<u16>,
    peers: Vec<NifPeerConfig>,
    replace_peers: bool,
}

#[rustler::nif]
fn list_devices() -> NifResult<(Atom, Vec<String>)> {
    Ok((atom::ok(), to_term_error(Device::list(BACKEND))?
        .iter()
        .map(|iname| iname.as_str_lossy().to_string())
        .collect()))
}

#[rustler::nif]
fn get_device(name: &str) -> NifResult<(Atom, NifDevice)> {
    let iname = parse_iname(name)?;
    let device = to_term_error(Device::get(&iname, BACKEND))?;

    Ok((atom::ok(), device.into()))
}

#[rustler::nif]
fn set_device(config: NifDeviceConfig, name: &str) -> NifResult<Atom> {
    let iname = parse_iname(name)?;
    let device: DeviceUpdate = config.try_into()?;

    to_term_error(device.apply(&iname, BACKEND))?;

    Ok(atom::ok())
}

#[rustler::nif]
fn delete_device(name: &str) -> NifResult<Atom> {
    let iname = parse_iname(name)?;
    let device = to_term_error(Device::get(&iname, BACKEND))?;

    to_term_error(device.delete())?;

    Ok(atom::ok())
}

#[rustler::nif]
fn remove_peer(name: &str, public_key: NifKey) -> NifResult<Atom> {
    let iname = parse_iname(name)?;
    let device = DeviceUpdate::new().remove_peer_by_key(&vec_to_key(public_key)?);

    to_term_error(device.apply(&iname, BACKEND))?;

    Ok(atom::ok())
}

#[rustler::nif]
fn add_peer(name: &str, peer: NifPeerConfig) -> NifResult<Atom> {
    let iname = parse_iname(name)?;
    let device = DeviceUpdate::new().add_peer(peer.try_into()?);

    to_term_error(device.apply(&iname, BACKEND))?;

    Ok(atom::ok())
}

fn parse_iname(name: &str) -> NifResult<InterfaceName> {
    to_term_error(name.parse())
}

pub(crate) fn to_term_error<T>(res: Result<T, impl ToString>) -> NifResult<T> {
    res.map_err(|e| Error::Term(Box::new(e.to_string())))
}
