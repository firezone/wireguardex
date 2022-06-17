defmodule Wireguardex.DeviceConfigBuilder do
  @moduledoc ~S"""
  This module contains functions to create a `DeviceConfig` which itself
  represents a configuration that can be set on a Wireguard device (interface).

  If an interface exists, the configuration is applied _on top_ of the existing
  interface's settings. Missing settings are not overwritten or set to defaults.

  To crate the default config use `device_config/0` then you can apply each
  function in this module to specify the details.

  ## Examples

      iex> res=
      ...> device_config()
      ...> |> listen_port(1234)
      ...> |> fwmark(11111)
      ...> |> Wireguardex.set_device("wg1234")
      ...> Wireguardex.delete_device("wg1234")
      ...> res
      :ok
  """

  @doc """
  Creates the default configuration to be configured with the provided functions.
  """
  def device_config do
    %Wireguardex.DeviceConfig{}
  end

  @doc """
  The public encryption key to set on the interface.
  """
  def public_key(config_builder, public_key) do
    %{config_builder | public_key: public_key}
  end

  @doc """
  The private encryption key to set on the interface.
  """
  def private_key(config_builder, private_key) do
    %{config_builder | private_key: private_key}
  end

  @doc """
  The [fwmark](https://man7.org/linux/man-pages/man8/tc-fw.8.html) to set on the
  interface.
  """
  def fwmark(config_builder, fwmark) do
    %{config_builder | fwmark: fwmark}
  end

  @doc """
  The listening port for incoming connections to set on the interface.
  """
  def listen_port(config_builder, listen_port) do
    %{config_builder | listen_port: listen_port}
  end

  @doc """
  A list of peers with their own configurations to set on this interface.
  """
  def peers(config_builder, peers) do
    %{config_builder | peers: peers}
  end

  @doc """
  If true, replace existing peer configurations on the interface.
  """
  def replace_peers(config_builder, replace_peers) do
    %{config_builder | replace_peers: replace_peers}
  end
end
