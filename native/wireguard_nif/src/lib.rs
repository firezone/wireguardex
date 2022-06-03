//! nif bindings for wireguard

mod device;
mod key;
mod peer;

use device::{delete_device, get_device, list_devices, remove_peer, set_device, add_peer};
use key::{generate_preshared_key, generate_private_key, get_public_key};

rustler::init!(
    "Elixir.Wireguardex",
    [
        list_devices,
        get_device,
        set_device,
        delete_device,
        remove_peer,
        generate_private_key,
        generate_preshared_key,
        get_public_key,
        add_peer,
    ]
);
