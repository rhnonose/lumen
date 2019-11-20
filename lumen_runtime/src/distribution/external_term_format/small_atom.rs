use liblumen_alloc::erts::exception::Exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use super::atom::bytes_len_try_into_term;
use super::u8;

pub fn decode<'a>(
    process: &Process,
    safe: bool,
    bytes: &'a [u8],
) -> Result<(Term, &'a [u8]), Exception> {
    let (len_u8, after_len_bytes) = u8::decode(process, bytes)?;
    let len_usize = len_u8 as usize;

    bytes_len_try_into_term(process, safe, after_len_bytes, len_usize)
}
