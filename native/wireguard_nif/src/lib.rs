//! NIF implementations for wireguard

use std::collections::HashMap;

use rustler::Term;

#[rustler::nif]
fn set(_config: HashMap<Term, &str>, _iname: &str) {}

#[rustler::nif]
fn show(_subcommand: &str, _iname: &str) {}

rustler::init!("Elixir.WireguardEx", [set, show]);
