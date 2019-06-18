extern crate image;
#[macro_use]
extern crate rustler;

mod imago;

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
    }
}

rustler_export_nifs! {
    "Elixir.Imago.Native",
    [
    ("read_pixels", 1, imago::readers::read_pixels),
    ("read_pixels_rgba", 1, imago::readers::read_pixels_rgba),
    ("read_pixels_rgb", 1, imago::readers::read_pixels_rgb),
    ("read_pixels_red", 1, imago::readers::read_pixels_red),
    ("read_pixels_green", 1, imago::readers::read_pixels_green),
    ("read_pixels_blue", 1, imago::readers::read_pixels_blue),
    ("read_pixels_alpha", 1, imago::readers::read_pixels_alpha),
    ("get_fingerprint", 1, imago::fingerprints::get_fingerprint),
    ("get_fingerprint_8x8", 1, imago::fingerprints::get_fingerprint_8x8),
    ("get_fingerprint_4x4", 1, imago::fingerprints::get_fingerprint_4x4),
    ("flatten_as_jpg", 1, imago::fs::flatten_as_jpg),
    ("threshold", 2, imago::dithers::threshold),
    ("dither_floyd_steinberg", 2, imago::dithers::dither_floyd_steinberg),
    ("dither_bayer", 2, imago::dithers::dither_bayer)
    ],
    None
}