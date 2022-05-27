defmodule WireguardExTest do
  use ExUnit.Case
  doctest WireguardEx

  test "NIFs loaded" do
    WireguardEx.list_devices()
  end
end
