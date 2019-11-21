use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception::Exception;
use liblumen_alloc::erts::term::prelude::*;
use liblumen_alloc::erts::Process;

use super::{arc_node, u32, u8, Pid};

pub fn decode_pid<'a>(
    process: &Process,
    safe: bool,
    bytes: &'a [u8],
) -> Result<(Pid, &'a [u8]), Exception> {
    let (arc_node, after_node_bytes) = arc_node::decode(process, safe, bytes)?;
    let (id, after_id_bytes) = u32::decode(process, after_node_bytes)?;
    let (serial, after_serial_bytes) = u32::decode(process, after_id_bytes)?;
    // TODO use creation to differentiate respawned nodes
    let (_creation, after_creation_bytes) = u8::decode(process, after_serial_bytes)?;

    let pid = Pid::new(arc_node, id, serial).map_err(|_| badarg!(process))?;

    Ok((pid, after_creation_bytes))
}

pub fn decode_term<'a>(
    process: &Process,
    safe: bool,
    bytes: &'a [u8],
) -> Result<(Term, &'a [u8]), Exception> {
    decode_pid(process, safe, bytes)
        .map(|(pid, after_pid_bytes)| (pid.clone_to_process(process), after_pid_bytes))
}
