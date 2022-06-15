//! nif bindings for generating wireguard keys

use rustler::{types::atom, Atom, NifResult};
use wireguard_control::Key;

use crate::device::to_term_error;

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
fn get_public_key(key: &str) -> NifResult<(Atom, String)> {
    let key = from_base64(key)?;

    Ok((atom::ok(), key.get_public().to_base64()))
}

pub(crate) fn from_base64(key: &str) -> NifResult<Key> {
    to_term_error(Key::from_base64(key))
}
