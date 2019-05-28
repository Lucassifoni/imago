defmodule Imago.MixProject do
  use Mix.Project

  def project do
    [
      app: :imago,
      version: "0.1.0",
      description: "Image manipulation at the pixel level, fingerprinting, [...]",
      source_url: "https://github.com/Lucassifoni/imago",
      package: [
        name: "imago",
        licenses: ["MIT"],
        links: %{
          "github" => "https://github.com/Lucassifoni/imago",
        },
        files:  ~w(lib priv .formatter.exs mix.exs README* readme* LICENSE* native test_image.jpg license* CHANGELOG* changelog* src)
      ],
      elixir: "~> 1.8",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: [imago: [
        # mode: (if Mix.env() == :prod, do: :release, else: :debug)
      ]]
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.20.0"}
    ]
  end
end
