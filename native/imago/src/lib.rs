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
    ("n_read_pixels", 1, read_pixels),
    ("n_read_pixels_rgba", 1, read_pixels_rgba),
    ("n_read_pixels_rgb", 1, read_pixels_rgb),
    ("n_read_pixels_red", 1, read_pixels_red),
    ("n_read_pixels_green", 1, read_pixels_green),
    ("n_read_pixels_blue", 1, read_pixels_blue),
    ("n_read_pixels_alpha", 1, read_pixels_alpha),
    ("n_get_fingerprint", 1, get_fingerprint),
    ("n_get_fingerprint", 1, get_fingerprint),
    ("n_get_fingerprint_8x8", 1, get_fingerprint_8x8),
    ("n_get_fingerprint_4x4", 1, get_fingerprint_4x4),
    ("n_flatten_as_jpg", 1, flatten_as_jpg),
    ("n_threshold", 2, threshold),
    ("n_dither_floyd_steinberg", 2, dither_floyd_steinberg),
    ("n_dither_bayer", 2, dither_bayer)
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

fn threshold<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let th: u8 = args[1].decode()?;
        let mut out: Vec<u8> = Vec::new();
        let w = f.width();
        let h = f.height();
        for pixel in f.pixels() {
            if luminance(pixel.2.data[0], pixel.2.data[1], pixel.2.data[2]) > th {
                out.push(255);
            } else {
                out.push(0)
            }
        }
        return Ok((atoms::ok(), (w, h, out)).encode(env));
    }
    Err(rustler::Error::Atom("Error"))
}

fn dither_floyd_steinberg<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let th: u8 = args[1].decode()?;
        let w = f.width();
        let h = f.height();
        let mut out: Vec<u8> = Vec::new();
        for pixel in f.pixels() {
            out.push(luminance(pixel.2[0], pixel.2[1], pixel.2[2]));
        }
        let mut err: u8;
        let mut new_pixel: u8;
        let mut ind: u32;
        for i in 0..h {
            for j in 0..w {
                ind = i * w + j;
                let lum = out[ind as usize];
                new_pixel = if lum < th { 0 } else { 255 };
                err = lum - new_pixel;
                out[ind as usize] = new_pixel;
                if j + 1 < w {
                    out[(ind + 1) as usize] += (err * 7) / 16;    
                }
                if i + 1 == h {
                    continue;
                }
                if j > 0 {
                    out[(ind + w - 1) as usize] += (err * 3) / 16;    
                    out[(ind + w) as usize] += (err * 5) / 16;
                }
                if j + 1 < w {
                    out[(ind + w + 1) as usize] += err / 16;
                }
            }
        }
        return Ok((atoms::ok(), (w, h, out)).encode(env));
    }
    Err(rustler::Error::Atom("Error"))
}

fn dither_bayer<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let bayer_threshold_map = [
            [15, 135, 45, 165],
            [195, 75, 225, 105],
            [60, 180, 30, 150],
            [240, 120, 210, 90]
        ];
        let w = f.width();
        let h = f.height();
        let th: u8 = args[1].decode()?;
        let mut out: Vec<u8> = Vec::new();
        for pixel in f.pixels() {
            let lum = luminance(pixel.2[0], pixel.2[1], pixel.2[2]);
            if (lum + bayer_threshold_map[(pixel.0 % 4) as usize][(pixel.1 % 4) as usize]) / 2 < th {
                out.push(0);
            } else {
                out.push(255);
            }
        }
        return Ok((atoms::ok(), (w, h, out)).encode(env));
    }
    Err(rustler::Error::Atom("Error"))
}

fn flatten_as_jpg<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let p: &'a str = args[0].decode()?;
        let path = format!("{}{}", p, ".jpg");
        let _res = f.save(path.clone());
        return Ok((atoms::ok(), path).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let w = f.width();
        let h = f.height();
        return Ok((atoms::ok(), (w, h, read_pixels_r(f))).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels_rgba<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    read_pixels(env, args)
}

fn read_pixels_rgb<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let w = f.width();
        let h = f.height();
        return Ok((atoms::ok(), (w, h, read_pixels_rgb_r(f))).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels_red<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let w = f.width();
        let h = f.height();
        return Ok((atoms::ok(), (w, h, read_pixels_red_r(f))).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels_green<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let w = f.width();
        let h = f.height();
        return Ok((atoms::ok(), (w, h, read_pixels_green_r(f))).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels_blue<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let w = f.width();
        let h = f.height();
        return Ok((atoms::ok(), (w, h, read_pixels_blue_r(f))).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels_alpha<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let w = f.width();
        let h = f.height();
        return Ok((atoms::ok(), (w, h, read_pixels_alpha_r(f))).encode(env));
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
        sign[index_2d_to_1d(pix.0 / 4, pix.1 / 4, 8) as usize] += luminance(pix.2.data[0], pix.2.data[1], pix.2.data[2]) as u32;
    }
    sign.iter().map(|a| *a / 16).collect()
}

fn compute_fingerprint_4x4(f: DynamicImage) -> Vec<u32> {
    let g = f.resize_exact(32, 32, FilterType::Nearest);
    let mut sign: [u32; 16] = [0; 16];
    for pix in g.pixels() {
        sign[index_2d_to_1d(pix.0 / 8, pix.1 / 8, 4) as usize] += luminance(pix.2.data[0], pix.2.data[1], pix.2.data[2]) as u32;
    }
    sign.iter().map(|a| *a / 64).collect()
}

fn index_2d_to_1d(x: u32, y: u32, rows: u32) -> u32 {
    rows * y + x
}

fn luminance(r: u8, g: u8, b: u8) -> u8 {
    ((r as f64 * 0.3) + (g as f64 * 0.59) + (b as f64 * 0.11)) as u8
}