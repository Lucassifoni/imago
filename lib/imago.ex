defmodule Imago do
  use Rustler, otp_app: :imago, crate: "imago"

  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)

  def read_pixels(_a), do: :erlang.nif_error(:nif_not_loaded)
end