use std::convert::TryInto;
use std::mem;

use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception::Exception;
use liblumen_alloc::erts::process::Process;

pub fn decode<'a>(process: &Process, bytes: &'a [u8]) -> Result<(u64, &'a [u8]), Exception> {
    const U64_BYTE_LEN: usize = mem::size_of::<u64>();

    if U64_BYTE_LEN <= bytes.len() {
        let (len_bytes, after_len_bytes) = bytes.split_at(U64_BYTE_LEN);
        let len_array = len_bytes.try_into().unwrap();
        let len_u64 = u64::from_be_bytes(len_array);

        Ok((len_u64, after_len_bytes))
    } else {
        Err(badarg!(process).into())
    }
}
