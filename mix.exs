defmodule WireguardEx.MixProject do
  use Mix.Project

  @source_url "https://github.com/firezone/wireguardex"
  @version "0.1.0"

  def project do
    [
      app: :wireguardex,
      version: @version,
      elixir: "~> 1.7",
      start_permanent: Mix.env() == :prod,
      compilers: Mix.compilers(),
      name: "wireguardex",
      package: package(),
      deps: deps(),
      aliases: aliases()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler_precompiled, "~> 0.5.1"},
      {:rustler, ">= 0.0.0", optional: true}
    ]
  end

  defp package do
    [
      description: "Native wireguard library implemented in Rust",
      maintainers: ["Andrew Rousset <andrew@firezone.dev>"],
      licenses: ["Apache 2.0"],
      files: ~w(lib native .formatter.exs README* LICENSE* mix.ecs checksums-*.exs),
      links: %{"GitHub" => @source_url}
    ]
  end

  defp aliases do
    [
      # force NIF compilation for tests
      test: [fn _ -> System.put_env("WIREGUARDEX_BUILD", "true") end, "test"]
    ]
  end
end
