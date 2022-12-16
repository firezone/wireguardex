defmodule Wireguardex.MixProject do
  use Mix.Project

  @source_url "https://github.com/firezone/wireguardex"
  @version "0.3.5"

  def project do
    [
      app: :wireguardex,
      version: @version,
      elixir: "~> 1.7",
      start_permanent: Mix.env() == :prod,
      compilers: Mix.compilers(),
      name: "Wireguardex",
      package: package(),
      deps: deps(),
      aliases: aliases(),
      docs: docs()
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
      {:rustler, ">= 0.0.0", optional: true},
      {:ex_doc, "~> 0.27", only: :dev, runtime: false},
      {:credo, "~> 1.6", only: [:dev, :test], runtime: false}
    ]
  end

  defp package do
    [
      description: "An Elixir library for configuring WireGuard interfaces via NIFs in Rust",
      organization: "firezonehq",
      maintainers: ["Jamil Bou Kheir <jamil@firezone.dev>"],
      licenses: ["Apache-2.0"],
      files: ~w(lib native .formatter.exs README* LICENSE* mix.exs checksum-*.exs),
      exclude_patterns: [".gitignore"],
      links: %{"GitHub" => @source_url}
    ]
  end

  defp docs do
    [
      main: "readme",
      extras: ["README.md": [title: "Overview"]],
      source_url: @source_url,
      source_ref: "v#{@version}",
      hompage_url: @source_url,
      formatters: ["html"]
    ]
  end

  defp aliases do
    [
      # force NIF compilation for tests
      test: [fn _ -> System.put_env("WIREGUARDNIF_BUILD", "true") end, "test"]
    ]
  end
end
