# Imago

Bindings (for my usage) to image-rs/image. Mainly because I didn't find any Elixir/Erlang library to iterate over pixels of compressed formats like Jpeg.
I'll need to at least open/process a bit/save images from rust.

At the time of this writing, it supports only one thing : getting a `Vec<u8>` from Rust from a file path.

```elixir
iex> Imago.read_pixels("/path/to/image.jpg|png|gif|ico|bmp|tiff") # Image formats are those of image-rs/image
{:ok,
 [131, 140, 157, 255, 130, 139, 156, 255, 131, 138, 156, 255, 131, 138, 156,
  255, 134, 139, 158, 255, 134, 139, 158, 255, 136, 139, 158, 255, 135, 138,
  157, 255, 135, 136, 156, 255, 135, 136, 156, 255, 136, 137, 157, 255, 136,
  137, 157, 255, ...]}
  
iex> Imago.read_pixels("/this/one/does/not/exist")
:error

iex> Imago.read_pixels("/this/one/is_a_text_file.txt")
:error
```

Available utilities:

`read_pixels/1` # {:ok, [r, g, b, a, r, g, b, a]}  
`read_pixels_rgba/1` # {:ok, [r, g, b, a, r, g, b, a]}     
`read_pixels_rgb/1` # {:ok, [r, g, b, r, g, b]}     
`read_pixels_red/1` # {:ok, [r, r, r]}  
`read_pixels_green/1` # {:ok, [g, g, g]}  
`read_pixels_blue/1` # {:ok, [b, b, b, b]}   
`read_pixels_alpha/1` # {:ok, [a, a, a, a]}  

