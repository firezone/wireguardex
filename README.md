# Wireguardex

An Elixir library for configuring WireGuard interfaces via NIFs implemented in
[Rust](https://rust-lang.org).

## Installation

The package can be installed by adding `wireguardex` to your list of dependencies
in `mix.exs`:

```elixir
def deps do
  [
    {:wireguardex, "~> 0.2.0"}
  ]
end
```

Wireguardex will try to download a precompiled NIF library. If you want to compile
your own NIF, you'll need to have Rust installed. The common option is to use
[Rustup](https://rustup.rs/).

To force compiliation you can set the environment variable `WIREGUARDNIF_BUILD`
to `true` or `1`. Or you can set the application env to force the NIF to compile:

```elixir
config :ruslter_precompiled, :force_build, wireguardex: true
```

## Tests

Running the tests will also require a Rust installation, as the NIF is compiled
locally before running the tests.
