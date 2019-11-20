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
use liblumen_alloc::erts::term::prelude::*;

use lumen_runtime_macros::native_implemented_function;

#[native_implemented_function(float/1)]
pub fn native(process: &Process, number: Term) -> exception::Result<Term> {
    if number.is_boxed_float() {
        Ok(number)
    } else {
        let f: f64 = number.try_into().map_err(|_| badarg!(process))?;

        process.float(f).map_err(|_| badarg!(process).into())
    }
}
