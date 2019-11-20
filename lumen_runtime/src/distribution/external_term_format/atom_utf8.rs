use liblumen_alloc::erts::exception::Exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use super::atom::{atom_bytes_to_term_bytes, bytes_len_try_into_atom};
use super::u16;

pub fn decode_atom<'a>(
    process: &Process,
    safe: bool,
    bytes: &'a [u8],
) -> Result<(Atom, &'a [u8]), Exception> {
    let (len_u16, after_len_bytes) = u16::decode(process, bytes)?;
    let len_usize = len_u16 as usize;

    bytes_len_try_into_atom(process, safe, after_len_bytes, len_usize)
}

pub fn decode_term<'a>(
    process: &Process,
    safe: bool,
    bytes: &'a [u8],
) -> Result<(Term, &'a [u8]), Exception> {
    decode_atom(process, safe, bytes).map(atom_bytes_to_term_bytes)
}
