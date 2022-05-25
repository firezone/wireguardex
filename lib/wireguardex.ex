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
    force_build: System.get_env("WIREGUARDEX_BUILD") in ["1", "true"],
    version: version

  def set(_config, _iname), do: error()
  def show(_subcommand, _iname), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
