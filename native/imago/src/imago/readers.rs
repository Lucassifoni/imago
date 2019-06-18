use image::{
    DynamicImage,
    GenericImageView,
};
use rustler::{Encoder, Env, NifResult, Term};

use atoms;
use imago::util::open_file_arg0;

pub fn read_pixels<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let w = f.width();
        let h = f.height();
        return Ok((atoms::ok(), (w, h, read_pixels_r(f))).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

pub fn read_pixels_rgba<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    read_pixels(env, args)
}

pub fn read_pixels_rgb<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let w = f.width();
        let h = f.height();
        return Ok((atoms::ok(), (w, h, read_pixels_rgb_r(f))).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

pub fn read_pixels_red<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let w = f.width();
        let h = f.height();
        return Ok((atoms::ok(), (w, h, read_pixels_red_r(f))).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

pub fn read_pixels_green<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let w = f.width();
        let h = f.height();
        return Ok((atoms::ok(), (w, h, read_pixels_green_r(f))).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

pub fn read_pixels_blue<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let w = f.width();
        let h = f.height();
        return Ok((atoms::ok(), (w, h, read_pixels_blue_r(f))).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}

pub fn read_pixels_alpha<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let w = f.width();
        let h = f.height();
        return Ok((atoms::ok(), (w, h, read_pixels_alpha_r(f))).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}


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