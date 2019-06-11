extern crate image;
#[macro_use]
extern crate rustler;

use std::path::Path;

use image::{DynamicImage, FilterType, GenericImageView};
use rustler::{Encoder, Env, Error, NifResult, Term};

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
    }
}

rustler_export_nifs! {
    "Elixir.Imago",
    [
    ("read_pixels", 1, read_pixels),
    ("read_pixels_rgba", 1, read_pixels_rgba),
    ("read_pixels_rgb", 1, read_pixels_rgb),
    ("read_pixels_red", 1, read_pixels_red),
    ("read_pixels_green", 1, read_pixels_green),
    ("read_pixels_blue", 1, read_pixels_blue),
    ("read_pixels_alpha", 1, read_pixels_alpha),
    ("get_fingerprint", 1, get_fingerprint),
    ("get_fingerprint", 1, get_fingerprint),
    ("get_fingerprint_8x8", 1, get_fingerprint_8x8),
    ("get_fingerprint_4x4", 1, get_fingerprint_4x4),
    ("flatten_as_jpg", 1, flatten_as_jpg)
    ],
    None
}

/* Utils */

fn open_file_arg0<'a>(arg0: Term<'a>) -> Result<DynamicImage, Error> {
    let path: &'a str = arg0.decode()?;
    if let Ok(f) = image::open(Path::new(path)) {
        return Ok(f);
    }
    Err(rustler::Error::Atom("error"))
}

/* Public */

fn flatten_as_jpg<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let p: &'a str = args[0].decode()?;
        let path = format!("{}{}", p, ".jpg");
        f.save(path.clone());
        return Ok((atoms::ok(), path).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), read_pixels_r(f)).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels_rgba<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    read_pixels(env, args)
}

fn read_pixels_rgb<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), read_pixels_rgb_r(f)).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels_red<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), read_pixels_red_r(f)).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels_green<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), read_pixels_green_r(f)).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels_blue<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), read_pixels_blue_r(f)).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels_alpha<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), read_pixels_alpha_r(f)).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

fn get_fingerprint<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    get_fingerprint_4x4(env, args)
}

fn get_fingerprint_4x4<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), compute_fingerprint_4x4(f).to_vec()).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

fn get_fingerprint_8x8<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), compute_fingerprint_8x8(f).to_vec()).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

/* Impls */

fn read_pixels_rgb_r(i: DynamicImage) -> Vec<u8> {
    let mut v = Vec::<u8>::new();
    for pixel in i.pixels() {
        v.push(pixel.2.data[0]);
        v.push(pixel.2.data[1]);
        v.push(pixel.2.data[2]);
    }
    v
}

fn read_pixels_red_r(i: DynamicImage) -> Vec<u8> {
    let mut v = Vec::<u8>::new();
    for pixel in i.pixels() {
        v.push(pixel.2.data[0])
    }
    v
}

fn read_pixels_green_r(i: DynamicImage) -> Vec<u8> {
    let mut v = Vec::<u8>::new();
    for pixel in i.pixels() {
        v.push(pixel.2.data[1])
    }
    v
}

fn read_pixels_blue_r(i: DynamicImage) -> Vec<u8> {
    let mut v = Vec::<u8>::new();
    for pixel in i.pixels() {
        v.push(pixel.2.data[2])
    }
    v
}

fn read_pixels_alpha_r(i: DynamicImage) -> Vec<u8> {
    let mut v = Vec::<u8>::new();
    for pixel in i.pixels() {
        v.push(pixel.2.data[3])
    }
    v
}


fn read_pixels_r(i: DynamicImage) -> Vec<u8> {
    let mut v = Vec::<u8>::new();
    for pixel in i.pixels() {
        for chan in &(pixel.2.data) {
            v.push(*chan)
        }
    }
    v
}

fn compute_fingerprint_8x8(f: DynamicImage) -> Vec<u32> {
    let g = f.resize_exact(32, 32, FilterType::Nearest);
    let mut sign: [u32; 64] = [0; 64];
    for pix in g.pixels() {
        sign[index_2d_to_1d(pix.0 / 4, pix.1 / 4, 8) as usize] += pixel_luminance(pix.2.data[0], pix.2.data[1], pix.2.data[2]) as u32;
    }
    sign.iter().map(|a| *a / 16).collect()
}

fn compute_fingerprint_4x4(f: DynamicImage) -> Vec<u32> {
    let g = f.resize_exact(32, 32, FilterType::Nearest);
    let mut sign: [u32; 16] = [0; 16];
    for pix in g.pixels() {
        sign[index_2d_to_1d(pix.0 / 8, pix.1 / 8, 4) as usize] += pixel_luminance(pix.2.data[0], pix.2.data[1], pix.2.data[2]) as u32;
    }
    sign.iter().map(|a| *a / 64).collect()
}

fn index_2d_to_1d(x: u32, y: u32, rows: u32) -> u32 {
    rows * y + x
}

fn pixel_luminance(r: u8, g: u8, b: u8) -> u8 {
    ((r as f64 * 0.3) + (g as f64 * 0.59) + (b as f64 * 0.11)) as u8
}