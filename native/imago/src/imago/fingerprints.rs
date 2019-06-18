use image::{
    DynamicImage,
    FilterType,
    GenericImageView,
};
use rustler::{Encoder, Env, NifResult, Term};

use atoms;
use imago::image::pixel_luminance;
use imago::util::{index_2d_to_1d, open_file_arg0};

pub fn get_fingerprint<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    get_fingerprint_4x4(env, args)
}

pub fn get_fingerprint_4x4<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), compute_fingerprint_4x4(f).to_vec()).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

pub fn get_fingerprint_8x8<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), compute_fingerprint_8x8(f).to_vec()).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

fn compute_fingerprint_8x8(f: DynamicImage) -> Vec<u32> {
    let g = f.resize_exact(32, 32, FilterType::Nearest);
    let mut sign: [u32; 64] = [0; 64];
    for pix in g.pixels() {
        sign[index_2d_to_1d(pix.0 / 4, pix.1 / 4, 8) as usize] += pixel_luminance(pix) as u32;
    }
    sign.iter().map(|a| *a / 16).collect()
}

fn compute_fingerprint_4x4(f: DynamicImage) -> Vec<u32> {
    let g = f.resize_exact(32, 32, FilterType::Nearest);
    let mut sign: [u32; 16] = [0; 16];
    for pix in g.pixels() {
        sign[index_2d_to_1d(pix.0 / 8, pix.1 / 8, 4) as usize] += pixel_luminance(pix) as u32;
    }
    sign.iter().map(|a| *a / 64).collect()
}