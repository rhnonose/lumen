use std::convert::TryInto;

use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

pub fn binary_to_string(process: &Process, binary: Term) -> exception::Result<String> {
    match binary.decode()? {
        TypedTerm::HeapBinary(heap_binary) => {
            heap_binary.try_into().map_err(|_| badarg!(process).into())
        }
        TypedTerm::SubBinary(subbinary) => {
            subbinary.try_into().map_err(|_| badarg!(process).into())
        }
        TypedTerm::ProcBin(process_binary) => process_binary
            .try_into()
            .map_err(|_| badarg!(process).into()),
        TypedTerm::MatchContext(match_context) => match_context
            .try_into()
            .map_err(|_| badarg!(process).into()),
        _ => Err(badarg!(process).into()),
    }
}
