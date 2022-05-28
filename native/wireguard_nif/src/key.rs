use rustler::{Error, NifResult};
use wireguard_control::Key;

pub(crate) fn from_base64(key: &str) -> NifResult<Key> {
    // Parse a base64 encoded key string into a valid Key struct.
    // Log an error with the invalid key string if it fails and map
    // to a rustler error.
    Key::from_base64(key).map_err(|_e| {
        eprintln!("[Error] Invalid key format (base64): {0}", key);
        Error::BadArg
    })
}
