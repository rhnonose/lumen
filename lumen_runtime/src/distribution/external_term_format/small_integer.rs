use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception::Exception;
use liblumen_alloc::erts::term::prelude::*;
use liblumen_alloc::erts::Process;

use super::{u8, Tag};

pub fn decode<'a>(process: &Process, bytes: &'a [u8]) -> Result<(Term, &'a [u8]), Exception> {
    let (small_integer_u8, after_small_integer_bytes) = u8::decode(process, bytes)?;
    let integer = process.integer(small_integer_u8)?;

    Ok((integer, after_small_integer_bytes))
}

pub fn decode_tagged_u8<'a>(
    process: &Process,
    bytes: &'a [u8],
) -> Result<(u8, &'a [u8]), Exception> {
    let (tag, after_tag_bytes) = Tag::decode(process, bytes)?;

    match tag {
        Tag::SmallInteger => u8::decode(process, after_tag_bytes),
        _ => Err(badarg!(process).into()),
    }
}
