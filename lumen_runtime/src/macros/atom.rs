macro_rules! boolean_infix_operator {
    ($process: ident, $left:ident, $right:ident, $operator:tt) => {{
        use std::convert::TryInto;

        use liblumen_alloc::erts::term::prelude::Encoded;

        let left_bool: bool = $left.decode()?.try_into().map_err(|_| liblumen_alloc::badarg!($process))?;
        let right_bool: bool = $right.decode()?.try_into().map_err(|_| liblumen_alloc::badarg!($process))?;
        let output_bool = left_bool $operator right_bool;

        Ok(output_bool.into())
    }};
}
