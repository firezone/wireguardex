defmodule Wireguardex.PeerInfo do
  @moduledoc """ 
  Documentation for Wireguardex.PeerInfo
  """

  alias Wireguardex.PeerConfig
  alias Wireguardex.PeerStats

  defstruct config: %PeerConfig{},
            stats: %PeerStats{}
end
