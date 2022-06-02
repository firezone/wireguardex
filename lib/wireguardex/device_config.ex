defmodule Wireguardex.DeviceConfig do
  @moduledoc """
  `DeviceConfig` represents a configuration that can be set on a Wireguard
  device (interface).

  If an interface exists, the configuration is applied _on top_ of the existing
  interface's settings. Missing settings are not overwritten or set to defaults.

  * `public_key` The public encryption key to set on the interface.
  * `private_key` The private encryption key to set on the interface.
  * `fwmark` The [fwmark](https://man7.org/linux/man-pages/man8/tc-fw.8.html) to
    set on the interface.
  * `listen_port` The listening port for incoming connections to set on the interface.
  * `peers` A list of peers with their own configurations to set on this interface.
  * `replace_peers` If true, replace existing peer configurations on the interface.
    If false, modify existing peer configurations or append them to the interface.
  """
  defstruct public_key: nil,
            private_key: nil,
            fwmark: nil,
            listen_port: nil,
            peers: [],
            replace_peers: false
end
