//! nif bindings for wireguard

mod device;
mod key;
mod peer;

rustler::init!("Elixir.Wireguardex");
