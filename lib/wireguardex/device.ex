defmodule Wireguardex.Device do
  @moduledoc """ 
  Documentation for Wireguardex.Device
  """
  defstruct name: "",
            public_key: nil,
            private_key: nil,
            fwmark: nil,
            listen_port: nil,
            peers: [],
            linked_name: nil
end
