defmodule Wireguardex.PeerStats do
  @moduledoc """
  `PeerStats` represent a peer's statistics from the current session.

  These are the attributes of a peer that will change over time; to get updated
  stats re-read the information from the interface.

  * `last_handshake_time` Timestamp of the last handshake/rekey with this peer.
  * `rx_bytes` Number of bytes received from this peer.
  * `tx_bytes` Number of bytes transmitted to this peer.
  """
  defstruct last_handshake_time: nil,
            rx_bytes: 0,
            tx_bytes: 0
end
