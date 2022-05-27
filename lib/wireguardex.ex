defmodule WireguardEx do
  @moduledoc """
  Documentation for `WireguardEx`.
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

  def list_devices(), do: error()
  def get_device(_name), do: error()
  def delete_device(_name), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
