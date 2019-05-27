defmodule Imago do
  use Rustler, otp_app: :imago, crate: "imago"

  def read_pixels(_a), do: :erlang.nif_error(:nif_not_loaded)
  def read_pixels_rgba(_a), do: :erlang.nif_error(:nif_not_loaded)
  def read_pixels_rgb(_a), do: :erlang.nif_error(:nif_not_loaded)
  def read_pixels_red(_a), do: :erlang.nif_error(:nif_not_loaded)
  def read_pixels_green(_a), do: :erlang.nif_error(:nif_not_loaded)
  def read_pixels_blue(_a), do: :erlang.nif_error(:nif_not_loaded)
  def read_pixels_alpha(_a), do: :erlang.nif_error(:nif_not_loaded)
end