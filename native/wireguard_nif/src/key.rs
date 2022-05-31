//! nif bindings for generating wireguard keys

use rustler::{Error, NifResult};
use wireguard_control::Key;

#[rustler::nif]
fn generate_private_key() -> String {
    let key = Key::generate_private();

    key.to_base64()
}

#[rustler::nif]
fn generate_preshared_key() -> String {
    let key = Key::generate_preshared();

    key.to_base64()
}

#[rustler::nif]
fn get_public_key(key: &str) -> NifResult<String> {
    let key = from_base64(key)?;

    Ok(key.get_public().to_base64())
}

pub(crate) fn from_base64(key: &str) -> NifResult<Key> {
    Key::from_base64(key).map_err(|e| Error::Term(Box::new(e.to_string())))
}
