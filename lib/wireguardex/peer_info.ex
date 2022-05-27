defmodule WireguardEx.PeerInfo do
  @moduledoc """ 
  Documentation for WireguardEx.PeerInfo
  """

  alias WireguardEx.PeerConfig
  alias WireguardEx.PeerStats

  defstruct config: %PeerConfig{},
            stats: %PeerStats{}
end
