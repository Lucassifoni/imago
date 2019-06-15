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
  defp n_flatten_as_jpg(_a), do: :erlang.nif_error(:nif_not_loaded)
  defp n_threshold(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
  defp n_dither_floyd_steinberg(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
  defp n_dither_bayer(_a, _b), do: :erlang.nif_error(:nif_not_loaded)

  def test_image() do
    (
      __ENV__.file
      |> Path.dirname) <> "/../test_image.jpg"
  end

  @doc """
  Since we use a 8x8 image, doctests would be polluted with
  lists of 4 * 64 integers. This ensures validity and conciseness.
  The real methods do return a list of integers.
  """
  def slice5({:ok, {_w, _h, result}}), do: {:ok, Enum.slice(result, 0..5)}
  def slicefp5({:ok, result}), do: {:ok, Enum.slice(result, 0..5)}

  @doc """
  Saves a PGM image to the given path
  """
  def save_pgm({width, height, data}, path) do
    { :ok, handle } = File.open(path, [:write])
    header =
       "P2\n"
    <> "#{width} #{height}\n"
    <> "255\n"
    IO.write(handle, header)
    Enum.chunk_every(data, width)
    |> Enum.each(fn row -> IO.write(handle, Enum.join(row, " ") <> "\n") end)
    File.close(handle)
  end

  @doc """
  Alias of read_pixels_rgba
  """
  def read_pixels(path) do
    n_read_pixels(path)
  end

  @doc """
  Re-saves an image as a jpeg.

      iex> Imago.test_image() |> Imago.flatten_as_jpg
      {:ok, "#{
    (
      __ENV__.file
      |> Path.dirname) <> "/../test_image.jpg"
  }.jpg"}
  """
  def flatten_as_jpg(path) do
    n_flatten_as_jpg(path)
  end

  @doc """
  Applies a threshold filter to an image

      iex> Imago.test_image() |> Imago.threshold(128) |> Imago.slice5
      {:ok, [255, 255, 255, 255, 255, 255]}
  """
  def threshold(path, threshold) do
    n_threshold(path, threshold)
  end

  @doc """
  Applies a floyd-steinberg filter to an image

      iex> Imago.test_image() |> Imago.dither_floyd_steinberg(128) |> Imago.slice5
      {:ok, [255, 255, 255, 255, 255, 255]}
  """
  def dither_floyd_steinberg(path, threshold) do
    n_dither_floyd_steinberg(path, threshold)
  end

  @doc """
  Applies a bayer filter to an image

      iex> Imago.test_image() |> Imago.dither_bayer(128) |> Imago.slice5
      {:ok, [0, 0, 0, 0, 0, 0]}
  """
  def dither_bayer(path, threshold) do
    n_dither_bayer(path, threshold)
  end

  @doc """
  Gets a list of rgba values

      iex> Imago.test_image() |> Imago.read_pixels_rgba |> Imago.slice5
      {:ok, [198, 198, 198, 255, 198, 198]}
  """
  def read_pixels_rgba(path) do
    n_read_pixels_rgba(path)
  end

  @doc """
  Gets a list of rgb values

      iex> Imago.test_image() |> Imago.read_pixels_rgb |> Imago.slice5
      {:ok, [198, 198, 198, 198, 198, 198]}
  """
  def read_pixels_rgb(path) do
    n_read_pixels_rgb(path)
  end

  @doc """
  Gets a list of red values

      iex> Imago.test_image() |> Imago.read_pixels_red |> Imago.slice5
      {:ok, [198, 198, 198, 198, 198, 198]}
  """
  def read_pixels_red(path) do
    n_read_pixels_red(path)
  end

  @doc """
  Gets a list of green values

      iex> Imago.test_image() |> Imago.read_pixels_green |> Imago.slice5
      {:ok, [198, 198, 198, 198, 198, 198]}
  """
  def read_pixels_green(path) do
    n_read_pixels_green(path)
  end

  @doc """
  Gets a list of blue values

      iex> Imago.test_image() |> Imago.read_pixels_blue |> Imago.slice5
      {:ok, [198, 198, 198, 198, 198, 198]}
  """
  def read_pixels_blue(path) do
    n_read_pixels_blue(path)
  end

  @doc """
  Gets a list of alpha values

      iex> Imago.test_image() |> Imago.read_pixels_alpha |> Imago.slice5
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

      iex> Imago.test_image() |> Imago.get_fingerprint_4x4 |> Imago.slicefp5
      {:ok, [207, 223, 174, 208, 225, 170]}
  """
  def get_fingerprint_4x4(path) do
    n_get_fingerprint_4x4(path)
  end

  @doc """
  Returns an image's fingerprint, sampled on a 8x8 luminance grid.

      iex> Imago.test_image() |> Imago.get_fingerprint_8x8 |> Imago.slicefp5
      {:ok, [198, 222, 222, 227, 209, 161]}
  """
  def get_fingerprint_8x8(path) do
    n_get_fingerprint_8x8(path)
  end

  @doc """
  Returns the average luminance of an image, sampled on a 4x4 grid,
  as an int.
  See get_fingerprint_4x4 for details

      iex> Imago.test_image() |> Imago.luminance
      {:ok, 192}
  """
  def luminance(path) do
    case get_fingerprint(path) do
      :error ->
        {:error, "Failed to fingerprint image at path #{path}"}
      {:ok, result} ->
        {
          :ok,
          (
            result
            |> Enum.sum) / length(result)
          |> round
        }
    end
  end
end