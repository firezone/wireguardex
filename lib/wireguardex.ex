defmodule Wireguardex do
  @moduledoc """
  Wireguardex is an Elixir library for configuring [WireGuard](https://wireguard.com) interfaces. It uses
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
    version: version,
    targets: [
      "aarch64-unknown-linux-gnu",
      "x86_64-unknown-linux-gnu",
      "aarch64-apple-darwin",
      "x86_64-apple-darwin"
    ]

  @type device :: Wireguardex.Device.t()
  @type device_config :: Wireguardex.DeviceConfig.t()
  @type peer_config :: Wireguardex.Peer.t()
  @type peer_stats :: Wireguardex.PeerStats.t()

  @type name :: String.t()
  @type key :: String.t()

  @doc """
  Get a list of interface names of WireGuard devices.

  Returns `{:ok, [...]}` if successful. `{:error, error_info}` will be returned
  if listing device interface names fails.
  """
  @spec list_devices() :: {:ok, [device()]} | {:error, String.t()}
  def list_devices, do: error()

  @doc """
  Get a `Device` by its interface name.

  Returns `{:ok, Device}` if successful. `{:error, error_info}` will be returned
  if getting the device fails.
  """
  @spec get_device(name()) :: {:ok, device()} | {:error, String.t()}
  def get_device(_name), do: error()

  @doc """
  Set a `Device` by its interface name using a `DeviceConfig`.

  Note if no device is present, a new one will be created for the given interface name.

  Returns `:ok` if successful. `{:error, error_info}` will be returned if setting
  the device fails.
  """
  @spec set_device(device_config(), name()) :: :ok | {:error, String.t()}
  def set_device(_device_config, _name), do: error()

  @doc """
  Delete a `Device` by its interface name.

  Returns `:ok` if successful. `{:error, error_info}` will be returned if deleting
  the device fails.
  """
  @spec delete_device(name()) :: :ok | {:error, String.t()}
  def delete_device(_name), do: error()

  @doc """
  Remove a peer from a `Device` by the peer's public key.

  Returns `:ok` if successful. `{:error, error_info}` will be returned if removing
  the peer from the device fails.
  """
  @spec remove_peer(name(), key()) :: :ok | {:error, String.t()}
  def remove_peer(_name, _public_key), do: error()

  @doc """
  Add a peer to a `Device`.

  Returns `:ok` if successful. `{:error, error_info}` will be returned if adding
  the peer to the device fails.
  """
  @spec add_peer(name(), peer_config()) :: :ok | {:error, String.t()}
  def add_peer(_name, _peer), do: error()

  @doc """
  Generates a random private key. It is returned as a base64 `string`.
  """

  @spec generate_private_key() :: key()
  def generate_private_key, do: error()

  @doc """
  Generates a random preshared key. It is returned as a base64 `string`.
  """

  @spec generate_preshared_key() :: key()
  def generate_preshared_key, do: error()

  @doc """
  Return a private key's public key as a base64 `string`.

  Returns `{:ok, public_key}` if successful. `{:error, error_info}` will be returned if
  if getting the public key fails.
  """
  @spec get_public_key(key()) :: key()
  def get_public_key(_private_key), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
