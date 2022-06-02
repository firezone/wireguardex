defmodule Wireguardex.PeerInfo do
  @moduledoc """
  `PeerInfo` represents all of the available information of a peer.

  This struct is a simple pair of a peer's configuration and stats.

  * `config` The configuration belonging to this peer.
  * `stats` The current statistics of this peer.
  """
  alias Wireguardex.PeerConfig
  alias Wireguardex.PeerStats

  defstruct config: %PeerConfig{},
            stats: %PeerStats{}
end
