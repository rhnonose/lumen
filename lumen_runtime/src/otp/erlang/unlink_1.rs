// wasm32 proptest cannot be compiled at the same time as non-wasm32 proptest, so disable tests that
// use proptest completely for wasm32
//
// See https://github.com/rust-lang/cargo/issues/4866
#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use lumen_runtime_macros::native_implemented_function;

use crate::registry::pid_to_process;

#[native_implemented_function(unlink/1)]
fn native(process: &Process, pid_or_port: Term) -> exception::Result<Term> {
    match pid_or_port.decode().unwrap() {
        TypedTerm::Pid(pid) => {
            if pid == process.pid() {
                Ok(true.into())
            } else {
                match pid_to_process(&pid) {
                    Some(pid_arc_process) => {
                        process.unlink(&pid_arc_process);
                    }
                    None => (),
                }

                Ok(true.into())
            }
        }
        TypedTerm::Port(_) => unimplemented!(),
        TypedTerm::ExternalPid(_) => unimplemented!(),
        TypedTerm::ExternalPort(_) => unimplemented!(),
        _ => Err(badarg!().into()),
    }
}
