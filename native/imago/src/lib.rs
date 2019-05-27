extern crate image;
#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;

use rustler::{Env, Term, NifResult, Encoder, Error, Binary};
use image::{GenericImageView, Pixel};
use std::fs::File;
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
    [("add", 2, add), ("read_pixels", 1, read_pixels)],
    None
}

fn read_pixels<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let path: &'a str = args[0].decode()?;
    if let Ok(f) = image::open(Path::new(path)) {
        let mut rgbax: Vec<u8> = Vec::new();
        for pixel in f.pixels() {
            for chan in &(pixel.2.data) {
                rgbax.push(*chan)
            }
        }
        return Ok((atoms::ok(), rgbax).encode(env))
    }
    Err(rustler::Error::Atom("error"))
}

fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let num1: i64 = args[0].decode()?;
    let num2: i64 = args[1].decode()?;

    Ok((atoms::ok(), num1 + num2).encode(env))
}
