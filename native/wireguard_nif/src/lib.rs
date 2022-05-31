//! nif bindings for wireguard

mod device;
mod key;
mod peer;

use device::{delete_device, get_device, list_devices, remove_peer, set_device};
use key::{generate_preshared_key, generate_private_key, get_public_key};

rustler::init!(
    "Elixir.WireguardEx",
    [
        list_devices,
        get_device,
        set_device,
        delete_device,
        remove_peer,
        generate_private_key,
        generate_preshared_key,
        get_public_key,
    ]
);
