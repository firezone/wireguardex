defmodule Wireguardex.PeerConfigBuilder do
  @moduledoc ~S"""
  This module contains functions to create a `PeerConfig`  which represents
  a peer's configuration of persistent attributes. They do not change over
  time and are part of the configuration of a device.

  To create the defualt config use `peer_config/0` then you can apply each
  function in this module to specify the config.

  ## Examples

      iex> {:ok, public_key} = Wireguardex.get_public_key(Wireguardex.generate_private_key())
      ...> res =
      ...> device_config()
      ...> |> peers([
      ...>   peer_config()
      ...>   |> Wireguardex.PeerConfigBuilder.public_key(public_key)
      ...>   |> endpoint("127.0.0.1:1234")
      ...>   |> preshared_key(Wireguardex.generate_preshared_key())
      ...>   |> persistent_keepalive_interval(60)
      ...>   |> allowed_ips(["127.0.0.1/16"])
      ...> ])
      ...> |> Wireguardex.set_device("wg12345")
      ...> Wireguardex.delete_device("wg12345")
      ...> res
      :ok
  """

  @doc """
  Creates the default peer configuration that you can then specify which each of
  the provided functions.
  """
  def peer_config do
    %Wireguardex.PeerConfig{}
  end

  @doc """
  The public key of the peer.
  """
  def public_key(peer_config, public_key) do
    %{peer_config | public_key: public_key}
  end

  @doc """
  The preshared key available to both peers (`nil` means no PSK is used).
  """
  def preshared_key(peer_config, preshared_key) do
    %{peer_config | preshared_key: preshared_key}
  end

  @doc """
  The endpoint this peer listens for connections on (`nil` means any).
  """
  def endpoint(peer_config, endpoint) do
    %{peer_config | endpoint: endpoint}
  end

  @doc """
  The interval for sending keepalive packets (`nil` means disabled).
  """
  def persistent_keepalive_interval(peer_config, persistent_keepalive_interval) do
    %{peer_config | persistent_keepalive_interval: persistent_keepalive_interval}
  end

  @doc """
  The allowed ip addresses this peer is allowed to have.
  """
  def allowed_ips(peer_config, allowed_ips) do
    %{peer_config | allowed_ips: allowed_ips}
  end
end
