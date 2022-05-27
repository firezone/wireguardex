//! nif bindings for wireguard keys

use rustler::NifStruct;

#[derive(NifStruct)]
#[module = "Elixir.WireguardEx.KeyPair"]
struct NifKeyPair {
    public: String,
    private: String,
}
