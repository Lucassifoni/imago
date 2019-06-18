defmodule Imago.IO do
  @doc """
  Saves a PGM image to the given path
  """
  def save_pgm({width, height, data}, path) do
    {:ok, handle} = File.open(path, [:write])
    header =
      "P2\n"
      <> "#{width} #{height}\n"
      <> "255\n"
    IO.write(handle, header)
    Enum.chunk_every(data, width)
    |> Enum.each(fn row -> IO.write(handle, Enum.join(row, " ") <> "\n") end)
    File.close(handle)
  end
end