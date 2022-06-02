defmodule Wireguardex.PeerConfig do
  @moduledoc """
  `PeerConfig` represents a peer's configuration of persistent attributes. They do
  not change over time and are part of the configuration of a device.

  * `public_key` The public key of the peer.
  * `preshared_key` The preshared key available to both peers (`nil` means no PSK
    is used).
  * `endpoint` The endpoint this peer listens for connections on (`nil` means any).
  * `persistent_keepalive_interval` The interval for sending keepalive packets
    (`nil` means disabled).
  * `allowed_ips` The allowed ip addresses this peer is allowed to have.
  """
  defstruct public_key: "",
            preshared_key: nil,
            endpoint: nil,
            persistent_keepalive_interval: nil,
            allowed_ips: []
end
