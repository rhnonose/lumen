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
use liblumen_alloc::{badkey, badmap};

use lumen_runtime_macros::native_implemented_function;

#[native_implemented_function(get/2)]
pub fn native(process: &Process, key: Term, map: Term) -> exception::Result<Term> {
    let result_map: Result<Boxed<Map>, _> = map.try_into();

    match result_map {
        Ok(map) => match map.get(key) {
            Some(value) => Ok(value.into()),
            None => Err(badkey!(process, key)),
        },
        Err(_) => Err(badmap!(process, map)),
    }
}
