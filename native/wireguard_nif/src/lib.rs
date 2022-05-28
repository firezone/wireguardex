//! nif bindings for wireguard

mod device;
mod error;
mod key;
mod peer;

use device::{list_devices, get_device, set_device, delete_device, remove_peer};

rustler::init!(
    "Elixir.WireguardEx",
    [list_devices, get_device, set_device, delete_device, remove_peer]
);
