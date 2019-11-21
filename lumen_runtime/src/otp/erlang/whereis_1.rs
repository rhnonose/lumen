// wasm32 proptest cannot be compiled at the same time as non-wasm32 proptest, so disable tests that
// use proptest completely for wasm32
//
// See https://github.com/rust-lang/cargo/issues/4866
#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use std::convert::TryInto;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;
use liblumen_alloc::{atom, badarg};

use lumen_runtime_macros::native_implemented_function;

use crate::registry;

#[native_implemented_function(whereis/1)]
pub fn native(process: &Process, name: Term) -> exception::Result<Term> {
    let atom: Atom = name.try_into().map_err(|_| badarg!(process))?;

    let option = registry::atom_to_process(&atom).map(|arc_process| arc_process.pid());

    let term = match option {
        Some(pid) => pid.encode()?,
        None => atom!("undefined"),
    };

    Ok(term)
}
