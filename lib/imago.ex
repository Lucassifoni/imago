defmodule Imago do
  use Rustler, otp_app: :imago, crate: "imago"

  defp n_read_pixels(_a), do: :erlang.nif_error(:nif_not_loaded)
  defp n_read_pixels_rgba(_a), do: :erlang.nif_error(:nif_not_loaded)
  defp n_read_pixels_rgb(_a), do: :erlang.nif_error(:nif_not_loaded)
  defp n_read_pixels_red(_a), do: :erlang.nif_error(:nif_not_loaded)
  defp n_read_pixels_green(_a), do: :erlang.nif_error(:nif_not_loaded)
  defp n_read_pixels_blue(_a), do: :erlang.nif_error(:nif_not_loaded)
  defp n_read_pixels_alpha(_a), do: :erlang.nif_error(:nif_not_loaded)
  defp n_get_fingerprint(_a), do: :erlang.nif_error(:nif_not_loaded)
  defp n_get_fingerprint_4x4(_a), do: :erlang.nif_error(:nif_not_loaded)
  defp n_get_fingerprint_8x8(_a), do: :erlang.nif_error(:nif_not_loaded)
  
  def test_image() do
    (__ENV__.file |> Path.dirname) <> "/../test_image.jpg"
  end

  @doc """
  Since we use a 8x8 image, doctests would be polluted with
  lists of 4 * 64 integers. This ensures validity and conciseness.
  The real methods do return a list of integers.
  """
  def test_checksum({:ok, result}), do: {:ok, Enum.slice(result, 0..5)}

  @doc """
  Alias of read_pixels_rgba
  """
  def read_pixels(path) do
    n_read_pixels(path)
  end

  @doc """
  Gets a list of rgba values
  iex> Imago.test_image() |> Imago.read_pixels_rgba |> Imago.test_checksum
  {:ok, [200, 200, 200, 255, 221, 221]}
  """
  def read_pixels_rgba(path) do
    n_read_pixels_rgba(path)
  end

  @doc """
  Gets a list of rgb values
  iex> Imago.test_image() |> Imago.read_pixels_rgb |> Imago.test_checksum
  {:ok, [200, 200, 200, 221, 221, 221]}
  """
  def read_pixels_rgb(path) do
    n_read_pixels_rgb(path)
  end

  @doc """
  Gets a list of red values
  iex> Imago.test_image() |> Imago.read_pixels_red |> Imago.test_checksum
  {:ok, [200, 221, 223, 228, 212, 160]}
  """
  def read_pixels_red(path) do
    n_read_pixels_red(path)
  end

  @doc """
  Gets a list of green values
  iex> Imago.test_image() |> Imago.read_pixels_green |> Imago.test_checksum
  {:ok, [200, 221, 223, 228, 212, 160]}
  """
  def read_pixels_green(path) do
    n_read_pixels_green(path)
  end

  @doc """
  Gets a list of blue values
  iex> Imago.test_image() |> Imago.read_pixels_blue |> Imago.test_checksum
  {:ok, [200, 221, 223, 228, 212, 160]}
  """
  def read_pixels_blue(path) do
    n_read_pixels_blue(path)
  end

  @doc """
  Gets a list of alpha values
  iex> Imago.test_image() |> Imago.read_pixels_alpha |> Imago.test_checksum
  {:ok, [255, 255, 255, 255, 255, 255]}
  """
  def read_pixels_alpha(path) do
    n_read_pixels_alpha(path)
  end

  @doc """
  Alias for get_fingerprint_4x4
  """
  def get_fingerprint(path) do
    n_get_fingerprint(path)
  end

  @doc """
  Returns an image's fingerprint, sampled on a 4x4 luminance grid
  iex> Imago.test_image() |> Imago.get_fingerprint_4x4 |> Imago.test_checksum
  {:ok, [208, 225, 180, 208, 225, 190]}
  """
  def get_fingerprint_4x4(path) do
    n_get_fingerprint_4x4(path)
  end

  @doc """
  Returns an image's fingerprint, sampled on a 8x8 luminance grid.
  iex> Imago.test_image() |> Imago.get_fingerprint_8x8 |> Imago.test_checksum
  {:ok, [200, 221, 222, 227, 212, 159]}
  """
  def get_fingerprint_8x8(path) do
    n_get_fingerprint_8x8(path)
  end

  @doc """
  Returns the average luminance of an image, sampled on a 4x4 grid,
  as an int.
  See get_fingerprint_4x4 for details
  iex> Imago.test_image() |> Imago.luminance
  {:ok, 207}
  """
  def luminance(path) do
    case get_fingerprint(path) do
      :error -> {:error, "Failed to fingerprint image at path #{path}"}
      {:ok, result} -> {:ok, (result |> Enum.sum) / length(result) |> round }
    end
  end
end