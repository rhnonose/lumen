use std::convert::TryInto;

use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use crate::process::spawn::options::Options;
use crate::scheduler::Scheduler;

pub(in crate::otp::erlang) fn native(
    process: &Process,
    options: Options,
    module: Term,
    function: Term,
    arguments: Term,
) -> exception::Result<Term> {
    let module_atom: Atom = module.try_into().map_err(|_| badarg!(process))?;
    let function_atom: Atom = function.try_into().map_err(|_| badarg!(process))?;

    let args = arguments.decode()?;
    if args.is_proper_list() {
        Scheduler::spawn_apply_3(process, options, module_atom, function_atom, arguments)
            .and_then(|spawned| spawned.to_term(process))
            .map_err(|e| e.into())
    } else {
        Err(badarg!(process).into())
    }
}
