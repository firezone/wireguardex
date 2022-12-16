# Wireguardex

## Overview

[![hex.pm](https://img.shields.io/hexpm/v/wireguardex.svg)](https://hex.pm/packages/wireguardex)
[![hex.pm](https://img.shields.io/hexpm/dt/wireguardex.svg)](https://hex.pm/packages/wireguardex)
[![hex.pm](https://img.shields.io/hexpm/l/wireguardex.svg)](https://hex.pm/packages/wireguardex)

Wireguardex is an Elixir library for configuring [WireGuard®](https://www.wireguard.com/) interfaces.

It is exposed as a native library via NIFs implemented in [Rust](https://rust-lang.org) using the [rustler](https://crates.io/crates/rustler) and [wireguard-control](https://docs.rs/wireguard-control/latest/wireguard_control/) crates.

Used by [Firezone](https://github.com/firezone/firezone) to manage WireGuard interfaces in Elixir.

## Getting started

Add `wireguardex` to your dependencies:

```elixir
def deps do
  [
    {:wireguardex, "~> 0.3"}
  ]
end
```

Then you can just use wireguardex to manage your wireguard interfaces:

```elixir
# Imports for cleanliness
import Wireguardex.DeviceConfigBuilder
import Wireguardex.PeerConfigBuilder
import Wireguardex, only: [set_device: 2]

interface_name = "wg0"
private_key = Wireguardex.generate_private_key()
{:ok, public_key} = Wireguardex.get_public_key(private_key)
listen_port = 58210
fwmark = 1234

:ok =
  device_config() # <-- Start configuring the devices
  # Here we set configuration for the device
  |> private_key(private_key)
  |> public_key(public_key)
  |> listen_port(listen_port)
  |> fwmark(fwmark)
  |> set_device(interface_name) # <-- This actually creates the interface
```

After creation you could also add peers:

```elixir
# Create a peer
peer =
  peer_config()
  |> public_key(public_key)
  |> preshared_key(Wireguardex.generate_preshared_key())
  |> endpoint("127.0.0.1:1234")
  |> persistent_keepalive_interval(30)
  |> allowed_ips(["255.0.0.0/24", "127.0.0.0/16"])

# Add peer to existing device
:ok = Wireguardex.add_peer(interface_name, peer)
```

And easily remove it afterwards using its public key:

```elixir
:ok = Wireguardex.remove_peer(interface_name, public_key)
```

To get information on an existing device:

```elixir
{:ok, device} = Wireguardex.get_device(interface_name)
```

Finally to delete a device:

```elixir
:ok = Wireguardex.delete_device(interface_name)
```

## Installation

The package can be installed by adding `wireguardex` to your list of dependencies
in `mix.exs`:

```elixir
def deps do
  [
    {:wireguardex, "~> 0.3"}
  ]
end
```

Wireguardex will try to download a precompiled NIF library. If you want to compile
your own NIF, you'll need to have Rust installed. The common option is to use
[Rustup](https://rustup.rs/).

To force compiliation you can set the environment variable `WIREGUARDNIF_BUILD`
to `true` or `1`. Or you can set the application env to force the NIF to compile:

```elixir
config :rustler_precompiled, :force_build, wireguardex: true
```

### Note about privileges

This library creates and modifies network interfaces. If you'd like to run this library as a non-root user, we recommend adding the `CAP_NET_ADMIN` Linux capability to the Erlang VM executable:

```sh
sudo setcap 'cap_net_admin+eip' <erlang_installation_path>/bin/beam.smp
```

If you're using [asdf-vm](https://asdf-vm.com/) to manage dependencies you can do:

```sh
sudo setcap 'cap_net_admin+eip' $(ls -1 `asdf where erlang 24.3.4`/erts-*/bin/beam.smp)
```

This can be handy for development and testing purposes.

**Note**: This will also give `CAP_NET_ADMIN` to any other Erlang programs using this `beam.smp` executable. If this is undesired, consider using a dedicated Erlang installation or `beam.smp` executable for this library.

## Features

* Manage WireGuard interfaces
* Doesn't require a WireGuard installation

## Tests

Running the tests in this library will also require a Rust installation, as the NIF is compiled
locally before running the tests.

Follow [these](https://www.rust-lang.org/learn/get-started) instructions to install Rust.

Then you can run `mix test` as long as you have the [user privileges to create interfaces](#note-about-privileges).

### Pre-commit

We use [pre-commit](https://pre-commit.com) to catch any static analysis issues before code is
committed. Install with Homebrew: `brew install pre-commit` or pip: `pip install pre-commit`.

## Acknowledgments

"WireGuard" and the "WireGuard" logo are registered trademarks of Jason A. Donenfeld.
