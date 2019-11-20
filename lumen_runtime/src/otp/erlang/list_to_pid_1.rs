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

use crate::distribution::nodes;

#[native_implemented_function(list_to_pid/1)]
pub fn native(process: &Process, string: Term) -> exception::Result<Term> {
    let cons: Boxed<Cons> = string.try_into().map_err(|_| badarg!(process))?;

    let prefix_tail = skip_char(process, cons, '<')?;
    let prefix_tail_cons: Boxed<Cons> = prefix_tail.try_into().map_err(|_| badarg!(process))?;

    let (node_id, node_tail) = next_decimal(process, prefix_tail_cons)?;
    let node_tail_cons: Boxed<Cons> = node_tail.try_into().map_err(|_| badarg!(process))?;

    let first_separator_tail = skip_char(process, node_tail_cons, '.')?;
    let first_separator_tail_cons: Boxed<Cons> = first_separator_tail
        .try_into()
        .map_err(|_| badarg!(process))?;

    let (number, number_tail) = next_decimal(process, first_separator_tail_cons)?;
    let number_tail_cons: Boxed<Cons> = number_tail.try_into().map_err(|_| badarg!(process))?;

    let second_separator_tail = skip_char(process, number_tail_cons, '.')?;
    let second_separator_tail_cons: Boxed<Cons> = second_separator_tail
        .try_into()
        .map_err(|_| badarg!(process))?;

    let (serial, serial_tail) = next_decimal(process, second_separator_tail_cons)?;
    let serial_tail_cons: Boxed<Cons> = serial_tail.try_into().map_err(|_| badarg!(process))?;

    let suffix_tail = skip_char(process, serial_tail_cons, '>')?;

    if suffix_tail.is_nil() {
        if node_id == nodes::node::id() {
            Pid::make_term(number, serial).map_err(|_| badarg!(process).into())
        } else {
            match nodes::id_to_arc_node(&node_id) {
                Some(arc_node) => process
                    .external_pid(arc_node, number, serial)
                    .map_err(|_| badarg!(process).into()),
                None => Err(badarg!(process).into()),
            }
        }
    } else {
        Err(badarg!(process).into())
    }
}

// Private

fn next_decimal(process: &Process, cons: Boxed<Cons>) -> exception::Result<(usize, Term)> {
    next_decimal_digit(process, cons)
        .and_then(|(first_digit, first_tail)| rest_decimal_digits(process, first_digit, first_tail))
}

fn next_decimal_digit(process: &Process, cons: Boxed<Cons>) -> exception::Result<(u8, Term)> {
    let head_char: char = cons.head.try_into().map_err(|_| badarg!(process))?;

    match head_char.to_digit(10) {
        Some(digit) => Ok((digit as u8, cons.tail)),
        None => Err(badarg!(process).into()),
    }
}

fn rest_decimal_digits(
    process: &Process,
    first_digit: u8,
    first_tail: Term,
) -> exception::Result<(usize, Term)> {
    match first_tail.try_into() {
        Ok(first_tail_cons) => {
            let mut acc_decimal: usize = first_digit as usize;
            let mut acc_tail = first_tail;
            let mut acc_cons: Boxed<Cons> = first_tail_cons;

            while let Ok((digit, tail)) = next_decimal_digit(process, acc_cons) {
                acc_decimal = 10 * acc_decimal + (digit as usize);
                acc_tail = tail;

                match tail.try_into() {
                    Ok(tail_cons) => acc_cons = tail_cons,
                    Err(_) => {
                        break;
                    }
                }
            }

            Ok((acc_decimal, acc_tail))
        }
        Err(_) => Ok((first_digit as usize, first_tail)),
    }
}

fn skip_char(process: &Process, cons: Boxed<Cons>, skip: char) -> exception::Result<Term> {
    let c: char = cons.head.try_into().map_err(|_| badarg!(process))?;

    if c == skip {
        Ok(cons.tail)
    } else {
        Err(badarg!(process).into())
    }
}
