defmodule Wireguardex.DeviceConfig do
  @moduledoc """ 
  Documentation for Wireguardex.DeviceConfig
  """
  defstruct public_key: nil,
            private_key: nil,
            fwmark: nil,
            listen_port: nil,
            peers: [],
            replace_peers: false
end
