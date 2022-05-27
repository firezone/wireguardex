defmodule WireguardEx.DeviceConfig do
  @moduledoc """ 
  Documentation for WireguardEx.DeviceConfig
  """
  defstruct public_key: nil,
            private_key: nil,
            fwmark: nil,
            listen_port: nil,
            peers: [],
            replace_peers: false
end
