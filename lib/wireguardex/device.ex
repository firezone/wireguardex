defmodule Wireguardex.Device do
  @moduledoc """
  `Device` represents all available information about a WireGuard device (interface).

  This struct contains the current configuration of the interface and the current
  configuration and info for all of its peers.

  * `name` The interface name of the device.
  * `public_key` The public encryption key of the interface.
  * `private_key` The private encryption key of the interface.
  * `fwmark` The [fwmark](https://man7.org/linux/man-pages/man8/tc-fw.8.html) of
    this interface.
  * `listen_port` The port to listen for incoming connections.
  * `peers` The list of all peers registered to this interface with their configs
    and stats.
  * `linked_name` The associated "real" name of the interface.
  """
  defstruct name: "",
            public_key: nil,
            private_key: nil,
            fwmark: nil,
            listen_port: nil,
            peers: [],
            linked_name: nil
end
