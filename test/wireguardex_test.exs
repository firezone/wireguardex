defmodule WireguardExTest do
  use ExUnit.Case
  doctest WireguardEx

  test "WireguardEx NIF loaded" do
    WireguardEx.set(%{}, "")
  end
end
