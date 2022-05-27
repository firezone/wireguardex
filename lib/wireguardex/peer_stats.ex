defmodule WireguardEx.PeerStats do
  @moduledoc """ 
  Documentation for WireguardEx.PeerStats
  """
  defstruct last_handshake_time: nil,
            rx_bytes: 0,
            tx_bytes: 0
end
