//! nif bindings for generating wireguard keys

use std::convert::TryInto;

use rustler::{types::atom, Atom, NifResult};
use wireguard_control::Key;

pub type NifKey = Vec<u8>;

pub(crate) fn vec_to_key(k: NifKey) -> NifResult<Key> {
    Ok(Key(k.try_into().map_err(|_| {
        rustler::Error::Term(Box::new("Key is either too long or too short"))
    })?))
}

#[rustler::nif]
fn generate_private_key() -> NifKey {
    Key::generate_private().0.to_vec()
}

#[rustler::nif]
fn generate_preshared_key() -> NifKey {
    Key::generate_preshared().0.to_vec()
}

#[rustler::nif]
fn get_public_key(key: NifKey) -> NifResult<(Atom, NifKey)> {
    Ok((atom::ok(), vec_to_key(key)?.get_public().0.to_vec()))
}
