defmodule Wireguardex do
  @moduledoc """
  Wireguardex is an Elixir library for configuring WireGuard interfaces. It uses
  [Rust](https://rust-lang.org) NIFs for performance and safety.

  This is the main module, providing the API for interface configuration and
  utilities such as key generation.
  """
  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links]["GitHub"]

  use RustlerPrecompiled,
    otp_app: :wireguardex,
    crate: "wireguard_nif",
    base_url: "#{github_url}/releases/download/v#{version}",
    force_build: System.get_env("WIREGUARDNIF_BUILD") in ["1", "true"],
    version: version

  @doc """
  Get a list of interfaces from wireguard.

  Returns `{:ok, [...]}` if successful. `{:error, error_info}` will be returned
  if listing the devices fails.
  """
  def list_devices, do: error()

  @doc """
  Get a `Device` by its interface name.

  Returns `{:ok, Device}` if successful. `{:error, error_info}` will be returned
  if getting the device fails.
  """
  def get_device(_name), do: error()

  @doc """
  Set a `Device` by its interface name using a `DeviceConfig`.

  Note if no device is present, a new one will be created for the given interface name.

  Returns `:ok` if successful. `{:error, error_info}` will be returned if setting
  the device fails.
  """
  def set_device(_name, _device_config), do: error()

  @doc """
  Delete a `Device` by its interface name.

  Returns `:ok` if successful. `{:error, error_info}` will be returned if deleting
  the device fails.
  """
  def delete_device(_name), do: error()

  @doc """
  Remove a peer from a `Device` by the peer's public key.

  Returns `:ok` if successful. `{:error, error_info}` will be returned if removing
  the peer from the device fails.
  """
  def remove_peer(_name, _public_key), do: error()

  @doc """
  Add a peer to a `Device`.

  Returns `:ok` if successful. `{:error, error_info}` will be returned if adding
  the peer to the device fails.
  """
  def add_peer(_name, _peer), do: error()

  @doc """
  Generates a random private key. It is returned as a base64 string.
  """
  def generate_private_key(), do: error()

  @doc """
  Generates a random preshared key. It is returned as a base64 string.
  """
  def generate_preshared_key(), do: error()

  @doc """
  Return a private key's public key as a base64 string.
  """
  def get_public_key(_private_key), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
