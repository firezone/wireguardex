//! nif bindings for wireguard

mod device;
mod error;
mod key;
mod peer;

use device::{delete_device, get_device, list_devices};

rustler::init!(
    "Elixir.WireguardEx",
    [list_devices, get_device, delete_device]
);
