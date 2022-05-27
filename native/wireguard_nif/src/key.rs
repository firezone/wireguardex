//! nif bindings for wireguard keys

use rustler::NifStruct;
use wireguard_control::{Key, KeyPair};

#[derive(NifStruct)]
#[module = "Elixir.WireguardEx.KeyPair"]
struct NifKeyPair {
    public: String,
    private: String,
}
