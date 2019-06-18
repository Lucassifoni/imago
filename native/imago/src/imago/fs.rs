use rustler::{Encoder, Env, NifResult, Term};

use atoms;
use imago::util::open_file_arg0;

pub fn flatten_as_jpg<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    if let Ok(f) = open_file_arg0(args[0]) {
        let p: &'a str = args[0].decode()?;
        let path = format!("{}{}", p, ".jpg");
        let _res = f.save(path.clone());
        return Ok((atoms::ok(), path).encode(env));
    }
    Err(rustler::Error::Atom("error"))
}