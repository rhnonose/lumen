// wasm32 proptest cannot be compiled at the same time as non-wasm32 proptest, so disable tests that
// use proptest completely for wasm32
//
// See https://github.com/rust-lang/cargo/issues/4866
#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use std::convert::TryInto;

use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::string::Encoding;
use liblumen_alloc::erts::term::prelude::*;

use lumen_runtime_macros::native_implemented_function;

#[native_implemented_function(binary_to_atom/2)]
pub fn native(process: &Process, binary: Term, encoding: Term) -> exception::Result<Term> {
    let _: Encoding = encoding.try_into().map_err(|_| badarg!(process))?;

    match binary.decode()? {
        TypedTerm::HeapBinary(heap_binary) => Atom::try_from_latin1_bytes(heap_binary.as_bytes())
            .map_err(|_| badarg!(process))?
            .encode(),
        TypedTerm::ProcBin(process_binary) => {
            Atom::try_from_latin1_bytes(process_binary.as_bytes())
                .map_err(|_| badarg!(process))?
                .encode()
        }
        TypedTerm::SubBinary(subbinary) => {
            if subbinary.is_binary() {
                let result_atom: Result<Atom, _> = if subbinary.is_aligned() {
                    let bytes = unsafe { subbinary.as_bytes_unchecked() };

                    Atom::try_from_latin1_bytes(bytes)
                } else {
                    let byte_vec: Vec<u8> = subbinary.full_byte_iter().collect();

                    Atom::try_from_latin1_bytes(&byte_vec)
                };

                match result_atom {
                    Ok(atom) => atom.encode(),
                    Err(_) => Err(badarg!(process).into()),
                }
            } else {
                Err(badarg!(process).into())
            }
        }
        _ => Err(badarg!(process).into()),
    }
}
