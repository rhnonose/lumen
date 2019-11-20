use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception::Exception;
use liblumen_alloc::erts::process::Process;

use super::{i32, u8, Tag};

pub fn decode<'a>(process: &Process, bytes: &'a [u8]) -> Result<(isize, &'a [u8]), Exception> {
    let (tag, after_tag_bytes) = Tag::decode(process, bytes)?;

    match tag {
        Tag::Integer => {
            let (i, after_i32_bytes) = i32::decode(process, after_tag_bytes)?;

            Ok((i as isize, after_i32_bytes))
        }
        Tag::SmallInteger => {
            let (u, after_u8_bytes) = u8::decode(process, after_tag_bytes)?;

            Ok((u as isize, after_u8_bytes))
        }
        _ => Err(badarg!(process).into()),
    }
}
