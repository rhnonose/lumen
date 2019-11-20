use num_bigint::Sign;

use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception::Exception;
use liblumen_alloc::erts::process::Process;

use super::u8;

pub fn decode<'a>(process: &Process, bytes: &'a [u8]) -> Result<(Sign, &'a [u8]), Exception> {
    let (sign_u8, after_sign_bytes) = u8::decode(process, bytes)?;
    let sign = byte_try_into_sign(process, sign_u8)?;

    Ok((sign, after_sign_bytes))
}

fn byte_try_into_sign(process: &Process, byte: u8) -> Result<Sign, Exception> {
    match byte {
        0 => Ok(Sign::Plus),
        1 => Ok(Sign::Minus),
        _ => Err(badarg!(process).into()),
    }
}
