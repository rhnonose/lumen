use std::convert::{TryFrom, TryInto};

use liblumen_alloc::erts::term::prelude::*;

pub struct Options {
    pub r#async: bool,
}

impl Options {
    fn put_option_term(&mut self, option: Term) -> Result<&Options, TryFromTermError> {
        let tuple: Boxed<Tuple> = option
            .try_into()
            .map_err(|_| TryFromTermError::ElementType)?;

        if tuple.len() == 2 {
            let atom: Atom = tuple[0]
                .try_into()
                .map_err(|_| TryFromTermError::KeywordKeyType)?;

            match atom.name() {
                "async" => {
                    self.r#async = tuple[1]
                        .try_into()
                        .map_err(|_| TryFromTermError::AsyncType)?;

                    Ok(self)
                }
                name => Err(TryFromTermError::KeywordKey(name).into()),
            }
        } else {
            Err(TryFromTermError::TupleSize)
        }
    }
}

impl Default for Options {
    fn default() -> Options {
        Options { r#async: false }
    }
}

impl TryFrom<Term> for Options {
    type Error = TryFromTermError;

    fn try_from(term: Term) -> Result<Self, Self::Error> {
        let mut options: Options = Default::default();
        let mut options_term = term;

        loop {
            match options_term.decode().unwrap() {
                TypedTerm::Nil => return Ok(options),
                TypedTerm::List(cons) => {
                    options.put_option_term(cons.head)?;
                    options_term = cons.tail;

                    continue;
                }
                _ => return Err(TryFromTermError::Type),
            }
        }
    }
}

pub enum TryFromTermError {
    AsyncType,
    ElementType,
    KeywordKeyType,
    KeywordKey(&'static str),
    TupleSize,
    Type,
}
