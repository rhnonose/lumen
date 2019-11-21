use liblumen_alloc::erts::exception::Exception;
use liblumen_alloc::erts::term::prelude::*;
use liblumen_alloc::erts::Process;

use super::super::u8;

pub fn decode<'a>(
    process: &Process,
    safe: bool,
    bytes: &'a [u8],
) -> Result<(Term, &'a [u8]), Exception> {
    let (len_u8, after_len_bytes) = u8::decode(process, bytes)?;

    super::decode(process, safe, after_len_bytes, len_u8 as usize)
}
