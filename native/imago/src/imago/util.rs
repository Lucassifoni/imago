use std::path::Path;

use image::DynamicImage;
use rustler::{Error, Term};

pub fn open_file_arg0<'a>(arg0: Term<'a>) -> Result<DynamicImage, Error> {
    let path: &'a str = arg0.decode()?;
    if let Ok(f) = image::open(Path::new(path)) {
        return Ok(f);
    }
    Err(rustler::Error::Atom("error"))
}

pub fn index_2d_to_1d(x: u32, y: u32, rows: u32) -> u32 {
    rows * y + x
}