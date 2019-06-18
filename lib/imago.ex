defmodule Imago do
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
  def slice5({:ok, {w, h, result}}), do: {:ok, {w, h, Enum.slice(result, 0..5)}}
  def slicefp5({:ok, result}), do: {:ok, Enum.slice(result, 0..5)}

  @doc """
  Alias of read_pixels_rgba
  """
  def read_pixels(path), do: Imago.Native.read_pixels(path)

  @doc """
  Re-saves an image as a jpeg.

      iex> Imago.test_image() |> Imago.flatten_as_jpg
      {:ok, "#{
    (
      __ENV__.file
      |> Path.dirname) <> "/../test_image.jpg"
  }.jpg"}
  """
  def flatten_as_jpg(path), do: Imago.Native.flatten_as_jpg(path)

  @doc """
  Applies a threshold filter to an image

      iex> Imago.test_image() |> Imago.threshold(128) |> Imago.slice5
      {:ok, {64, 64, [255, 255, 255, 255, 255, 255]}}
  """
  def threshold(path, threshold), do: Imago.Native.threshold(path, threshold)

  @doc """
  Applies a floyd-steinberg filter to an image

      iex> Imago.test_image() |> Imago.dither_floyd_steinberg(128) |> Imago.slice5
      {:ok, {64, 64, [255, 255, 255, 255, 255, 255]}}
  """
  def dither_floyd_steinberg(path, threshold), do: Imago.Native.dither_floyd_steinberg(path, threshold)

  @doc """
  Applies a bayer filter to an image

      iex> Imago.test_image() |> Imago.dither_bayer(128) |> Imago.slice5
      {:ok, {64, 64, [0, 0, 0, 0, 0, 0]}}
  """
  def dither_bayer(path, threshold), do: Imago.Native.dither_bayer(path, threshold)

  @doc """
  Gets a list of rgba values

      iex> Imago.test_image() |> Imago.read_pixels_rgba |> Imago.slice5
      {:ok, {64, 64, [198, 198, 198, 255, 198, 198]}}
  """
  def read_pixels_rgba(path), do: Imago.Native.read_pixels_rgba(path)

  @doc """
  Gets a list of rgb values

      iex> Imago.test_image() |> Imago.read_pixels_rgb |> Imago.slice5
      {:ok, {64, 64, [198, 198, 198, 198, 198, 198]}}
  """
  def read_pixels_rgb(path), do: Imago.Native.read_pixels_rgb(path)

  @doc """
  Gets a list of red values

      iex> Imago.test_image() |> Imago.read_pixels_red |> Imago.slice5
      {:ok, {64, 64, [198, 198, 198, 198, 198, 198]}}
  """
  def read_pixels_red(path), do: Imago.Native.read_pixels_red(path)

  @doc """
  Gets a list of green values

      iex> Imago.test_image() |> Imago.read_pixels_green |> Imago.slice5
      {:ok, {64, 64, [198, 198, 198, 198, 198, 198]}}
  """
  def read_pixels_green(path), do: Imago.Native.read_pixels_green(path)

  @doc """
  Gets a list of blue values

      iex> Imago.test_image() |> Imago.read_pixels_blue |> Imago.slice5
      {:ok, {64, 64, [198, 198, 198, 198, 198, 198]}}
  """
  def read_pixels_blue(path), do: Imago.Native.read_pixels_blue(path)

  @doc """
  Gets a list of alpha values

      iex> Imago.test_image() |> Imago.read_pixels_alpha |> Imago.slice5
      {:ok, {64, 64, [255, 255, 255, 255, 255, 255]}}
  """
  def read_pixels_alpha(path), do: Imago.Native.read_pixels_alpha(path)

  @doc """
  Alias for get_fingerprint_4x4
  """
  def get_fingerprint(path), do: Imago.Native.get_fingerprint(path)

  @doc """
  Returns an image's fingerprint, sampled on a 4x4 luminance grid

      iex> Imago.test_image() |> Imago.get_fingerprint_4x4 |> Imago.slicefp5
      {:ok, [207, 223, 174, 208, 225, 170]}
  """
  def get_fingerprint_4x4(path), do: Imago.Native.get_fingerprint_4x4(path)

  @doc """
  Returns an image's fingerprint, sampled on a 8x8 luminance grid.

      iex> Imago.test_image() |> Imago.get_fingerprint_8x8 |> Imago.slicefp5
      {:ok, [198, 222, 222, 227, 209, 161]}
  """
  def get_fingerprint_8x8(path), do: Imago.Native.get_fingerprint_8x8(path)

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