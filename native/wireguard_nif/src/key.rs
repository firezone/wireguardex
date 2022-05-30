use rustler::{Error, NifResult};
use wireguard_control::Key;

pub(crate) fn from_base64(key: &str) -> NifResult<Key> {
    Key::from_base64(key).map_err(|e| Error::Term(Box::new(e.to_string())))
}
