use std::convert::TryInto;

use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

pub fn charlist_to_string(process: &Process, list: Term) -> exception::Result<String> {
    match list.decode()? {
        TypedTerm::Nil => Ok("".to_string()),
        TypedTerm::List(boxed_cons) => boxed_cons.try_into().map_err(|_| badarg!(process).into()),
        _ => Err(badarg!(process).into()),
    }
}
