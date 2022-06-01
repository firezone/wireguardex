defmodule Wireguardex.PeerConfig do
  @moduledoc """ 
  Documentation for Wireguardex.PeerConfig
  """
  defstruct public_key: "",
            preshared_key: nil,
            endpoint: nil,
            persistent_keepalive_interval: nil,
            allowed_ips: []
end
