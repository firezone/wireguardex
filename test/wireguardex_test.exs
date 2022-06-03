defmodule WireguardExTest do
  use ExUnit.Case
  doctest WireguardEx

  test "set device" do
    interface_name = "wg0"
    private_key = WireguardEx.generate_private_key()
    public_key = WireguardEx.get_public_key(private_key)
    listen_port = 58210
    fwmark = 1234

    set_result =
      WireguardEx.set_device(interface_name, %WireguardEx.DeviceConfig{
        public_key: public_key,
        private_key: private_key,
        fwmark: fwmark,
        listen_port: listen_port
      })

    device = WireguardEx.get_device(interface_name)
    delete_result = WireguardEx.delete_device(interface_name)

    assert set_result == :ok
    assert delete_result == :ok
    assert device.name == interface_name
    assert device.public_key == public_key
    assert device.private_key == private_key
    assert device.fwmark == fwmark
    assert device.listen_port == listen_port
  end

  test "list devices" do
    interface_name = "wg1"
    set_result = WireguardEx.set_device(interface_name, %WireguardEx.DeviceConfig{})
    devices = WireguardEx.list_devices()
    delete_result = WireguardEx.delete_device(interface_name)

    assert set_result == :ok
    assert delete_result == :ok
    assert List.first(devices) == interface_name
  end

  test "add peers to device" do
    interface_name = "wg2"

    peers = [
      %WireguardEx.PeerConfig{
        public_key: WireguardEx.get_public_key(WireguardEx.generate_private_key()),
        preshared_key: WireguardEx.generate_preshared_key(),
        endpoint: "127.0.0.1:1234",
        persistent_keepalive_interval: 60,
        allowed_ips: ["192.168.0.0/24", "163.23.42.242/32"]
      },
      %WireguardEx.PeerConfig{
        public_key: WireguardEx.get_public_key(WireguardEx.generate_private_key()),
        preshared_key: WireguardEx.generate_preshared_key(),
        endpoint: "127.0.0.2:1234",
        persistent_keepalive_interval: 30,
        allowed_ips: ["255.0.0.0/24", "127.0.0.0/16"]
      }
    ]

    set_result = WireguardEx.set_device(interface_name, %WireguardEx.DeviceConfig{peers: peers})

    device = WireguardEx.get_device(interface_name)
    delete_result = WireguardEx.delete_device(interface_name)

    assert set_result == :ok
    assert delete_result == :ok
    assert List.first(device.peers).config == List.first(peers)
    assert List.last(device.peers).config == List.last(peers)
  end

  test "add peer to device after creation" do
    interface_name = "wg3"

    peer = %WireguardEx.PeerConfig{
      public_key: WireguardEx.get_public_key(WireguardEx.generate_private_key()),
      preshared_key: WireguardEx.generate_preshared_key(),
      endpoint: "127.0.0.1:1234",
      persistent_keepalive_interval: 60,
      allowed_ips: ["192.168.0.0/24", "163.23.42.242/32"]
    }

    set_result = WireguardEx.set_device(interface_name, %WireguardEx.DeviceConfig{})

    add_result = WireguardEx.add_peer(interface_name, peer)
    device = WireguardEx.get_device(interface_name)
    delete_result = WireguardEx.delete_device(interface_name)

    assert set_result == :ok
    assert add_result == :ok
    assert List.first(device.peers).config == peer
    assert delete_result == :ok
  end
end
