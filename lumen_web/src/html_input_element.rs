pub mod value_1;

use std::convert::TryInto;
use std::mem;

use web_sys::HtmlInputElement;

use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

// Private

fn from_term(
    process: &Process,
    term: Term,
) -> Result<&'static HtmlInputElement, exception::Exception> {
    let boxed: Boxed<Resource> = term.try_into().map_err(|_| badarg!(process))?;
    let html_input_element_reference: Resource = boxed.into();

    match html_input_element_reference.downcast_ref() {
        Some(html_input_element) => {
            let static_html_input_element: &'static HtmlInputElement =
                unsafe { mem::transmute::<&HtmlInputElement, _>(html_input_element) };

            Ok(static_html_input_element)
        }
        None => Err(badarg!(process).into()),
    }
}

fn module() -> Atom {
    Atom::try_from_str("Elixir.Lumen.Web.HTMLInputElement").unwrap()
}
