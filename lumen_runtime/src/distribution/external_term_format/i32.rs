use std::convert::TryInto;
use std::mem;

use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception::Exception;
use liblumen_alloc::erts::process::Process;

pub fn decode<'a>(process: &Process, bytes: &'a [u8]) -> Result<(i32, &'a [u8]), Exception> {
    if I32_BYTE_LEN <= bytes.len() {
        let (len_bytes, after_len_bytes) = bytes.split_at(I32_BYTE_LEN);
        let len_array = len_bytes.try_into().unwrap();
        let len_i32 = i32::from_be_bytes(len_array);

        Ok((len_i32, after_len_bytes))
    } else {
        Err(badarg!(process).into())
    }
}

const I32_BYTE_LEN: usize = mem::size_of::<i32>();
