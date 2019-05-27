extern crate image;
#[macro_use] extern crate rustler;

use rustler::{Env, Term, NifResult, Encoder, Error};
use image::{GenericImageView, DynamicImage};
use std::path::Path;

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
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

fn read_pixels<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), read_pixels_r(f)).encode(env))
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels_rgba<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    read_pixels(env, args)
}

fn read_pixels_rgb<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), read_pixels_rgb_r(f)).encode(env))
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels_red<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), read_pixels_red_r(f)).encode(env))
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels_green<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), read_pixels_green_r(f)).encode(env))
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels_blue<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), read_pixels_blue_r(f)).encode(env))
    }
    Err(rustler::Error::Atom("error"))
}

fn read_pixels_alpha<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        return Ok((atoms::ok(), read_pixels_alpha_r(f)).encode(env))
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
