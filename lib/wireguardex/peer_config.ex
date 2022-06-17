defmodule Wireguardex.PeerConfig do
  @moduledoc false
  defstruct public_key: "",
            preshared_key: nil,
            endpoint: nil,
            persistent_keepalive_interval: nil,
            allowed_ips: []

  @type t :: %__MODULE__{}
end
