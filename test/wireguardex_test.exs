defmodule WireguardexTest do
  use ExUnit.Case
  import Wireguardex.DeviceConfigBuilder, except: [public_key: 2]
  import Wireguardex.PeerConfigBuilder, except: [public_key: 2]
  import Wireguardex, only: [set_device: 2]
  doctest Wireguardex
  doctest Wireguardex.DeviceConfigBuilder
  doctest Wireguardex.PeerConfigBuilder

  test "set device" do
    interface_name = "wg0"
    private_key = Wireguardex.generate_private_key()
    {:ok, public_key} = Wireguardex.get_public_key(private_key)
    listen_port = 58210
    fwmark = 1234

    :ok =
      device_config()
      |> private_key(private_key)
      |> Wireguardex.PeerConfigBuilder.public_key(public_key)
      |> listen_port(listen_port)
      |> fwmark(fwmark)
      |> set_device(interface_name)

    {:ok, device} = Wireguardex.get_device(interface_name)
    :ok = Wireguardex.delete_device(interface_name)

    assert device.name == interface_name
    assert device.public_key == public_key
    assert device.private_key == private_key
    assert device.fwmark == fwmark
    assert device.listen_port == listen_port
  end

  test "list devices" do
    interface_name = "wg1"
    :ok = Wireguardex.set_device(%Wireguardex.DeviceConfig{}, interface_name)
    {:ok, devices} = Wireguardex.list_devices()
    :ok = Wireguardex.delete_device(interface_name)

    assert List.first(devices) == interface_name
  end

  test "add peers to device" do
    interface_name = "wg2"
    {:ok, public_key0} = Wireguardex.get_public_key(Wireguardex.generate_private_key())
    {:ok, public_key1} = Wireguardex.get_public_key(Wireguardex.generate_private_key())

    peers = [
      peer_config()
      |> Wireguardex.PeerConfigBuilder.public_key(public_key0)
      |> preshared_key(Wireguardex.generate_preshared_key())
      |> endpoint("127.0.0.1:1234")
      |> persistent_keepalive_interval(60)
      |> allowed_ips(["192.168.0.0/24", "163.23.42.242/32"]),
      peer_config()
      |> Wireguardex.PeerConfigBuilder.public_key(public_key1)
      |> preshared_key(Wireguardex.generate_preshared_key())
      |> endpoint("127.0.0.1:1234")
      |> persistent_keepalive_interval(30)
      |> allowed_ips(["255.0.0.0/24", "127.0.0.0/16"])
    ]

    :ok =
      device_config()
      |> peers(peers)
      |> set_device(interface_name)

    {:ok, device} = Wireguardex.get_device(interface_name)
    :ok = Wireguardex.delete_device(interface_name)

    assert List.first(device.peers).config == List.first(peers)
    assert List.last(device.peers).config == List.last(peers)
  end

  test "add peer to device after creation" do
    interface_name = "wg3"
    {:ok, public_key} = Wireguardex.get_public_key(Wireguardex.generate_private_key())

    peer = %Wireguardex.PeerConfig{
      public_key: public_key,
      preshared_key: Wireguardex.generate_preshared_key(),
      endpoint: "127.0.0.1:1234",
      persistent_keepalive_interval: 60,
      allowed_ips: ["192.168.0.0/24", "163.23.42.242/32"]
    }

    :ok =
      device_config()
      |> set_device(interface_name)

    :ok = Wireguardex.add_peer(interface_name, peer)
    {:ok, device} = Wireguardex.get_device(interface_name)
    :ok = Wireguardex.delete_device(interface_name)

    assert List.first(device.peers).config == peer
  end

  test "remove peer to device after creation" do
    interface_name = "wg4"
    {:ok, public_key} = Wireguardex.get_public_key(Wireguardex.generate_private_key())

    peer = %Wireguardex.PeerConfig{
      public_key: public_key,
      preshared_key: Wireguardex.generate_preshared_key(),
      endpoint: "127.0.0.1:1234",
      persistent_keepalive_interval: 60,
      allowed_ips: ["192.168.0.0/24", "163.23.42.242/32"]
    }

    :ok =
      device_config()
      |> peers([peer])
      |> set_device(interface_name)

    :ok = Wireguardex.remove_peer(interface_name, public_key)
    {:ok, device} = Wireguardex.get_device(interface_name)
    :ok = Wireguardex.delete_device(interface_name)

    assert device.peers == []
  end
end
