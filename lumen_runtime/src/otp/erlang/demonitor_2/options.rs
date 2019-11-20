use std::convert::{TryFrom, TryInto};

use liblumen_alloc::erts::term::prelude::*;

pub struct Options {
    pub flush: bool,
    pub info: bool,
}

impl Options {
    fn put_option_term(&mut self, option: Term) -> Result<&Self, TryFromTermError> {
        let option_atom: Atom = option
            .try_into()
            .map_err(|_| TryFromTermError::ElementType)?;

        match option_atom.name() {
            "flush" => {
                self.flush = true;

                Ok(self)
            }
            "info" => {
                self.info = true;

                Ok(self)
            }
            name => Err(TryFromTermError::AtomName(name)),
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            flush: false,
            info: false,
        }
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
            };
        }
    }
}

pub enum TryFromTermError {
    AtomName(&'static str),
    ElementType,
    Type,
}
