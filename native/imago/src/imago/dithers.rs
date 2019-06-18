use image::GenericImageView;
use rustler::{Env, NifResult, Term};
use rustler::Encoder;

use atoms;
use imago::image::pixel_luminance;
use imago::util::open_file_arg0;

pub fn threshold<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let th: u8 = args[1].decode()?;
        let mut out: Vec<u8> = Vec::new();
        let w = f.width();
        let h = f.height();
        for pixel in f.pixels() {
            if pixel_luminance(pixel) > th {
                out.push(255);
            } else {
                out.push(0)
            }
        }
        return Ok((atoms::ok(), (w, h, out)).encode(env));
    }
    Err(rustler::Error::Atom("Error"))
}

pub fn dither_floyd_steinberg<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let th: u8 = args[1].decode()?;
        let w = f.width();
        let h = f.height();
        let mut out: Vec<u8> = Vec::new();
        for pixel in f.pixels() {
            out.push(pixel_luminance(pixel));
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

pub fn dither_bayer<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
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
            let lum = pixel_luminance(pixel);
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