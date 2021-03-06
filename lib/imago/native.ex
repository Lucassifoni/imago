defmodule Imago.Native do
  use Rustler, otp_app: :imago, crate: "imago"

  def read_pixels(_a), do: :erlang.nif_error(:nif_not_loaded)
  def read_pixels_rgba(_a), do: :erlang.nif_error(:nif_not_loaded)
  def read_pixels_rgb(_a), do: :erlang.nif_error(:nif_not_loaded)
  def read_pixels_red(_a), do: :erlang.nif_error(:nif_not_loaded)
  def read_pixels_green(_a), do: :erlang.nif_error(:nif_not_loaded)
  def read_pixels_blue(_a), do: :erlang.nif_error(:nif_not_loaded)
  def read_pixels_alpha(_a), do: :erlang.nif_error(:nif_not_loaded)
  def get_fingerprint(_a), do: :erlang.nif_error(:nif_not_loaded)
  def get_fingerprint_4x4(_a), do: :erlang.nif_error(:nif_not_loaded)
  def get_fingerprint_8x8(_a), do: :erlang.nif_error(:nif_not_loaded)
  def flatten_as_jpg(_a), do: :erlang.nif_error(:nif_not_loaded)
  def threshold(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
  def dither_floyd_steinberg(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
  def dither_bayer(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end