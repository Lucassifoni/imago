# Imago

Bindings (for my usage) to image-rs/image. Mainly because I didn't find any Elixir/Erlang library to iterate over pixels of compressed formats like Jpeg.

At the time of this writing, it supports only one thing : getting a `Vec<u8>` from Rust with `r, g, b, a` ordering, from a file path.

```elixir
iex> Imago.read_pixels("/path/to/image.jpg|png|gif|ico|bmp|tiff") # Image formats are those of image-rs/image
{:ok,
 [131, 140, 157, 255, 130, 139, 156, 255, 131, 138, 156, 255, 131, 138, 156,
  255, 134, 139, 158, 255, 134, 139, 158, 255, 136, 139, 158, 255, 135, 138,
  157, 255, 135, 136, 156, 255, 135, 136, 156, 255, 136, 137, 157, 255, 136,
  137, 157, 255, ...]}
```