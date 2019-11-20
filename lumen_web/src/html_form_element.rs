pub mod element_2;

use std::convert::TryInto;
use std::mem;

use wasm_bindgen::JsCast;

use web_sys::{EventTarget, HtmlFormElement};

use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

// Private

fn from_term(
    process: &Process,
    term: Term,
) -> Result<&'static HtmlFormElement, exception::Exception> {
    let boxed: Boxed<Resource> = term.try_into().map_err(|_| badarg!(process))?;
    let resource_reference: Resource = boxed.into();

    if resource_reference.is::<EventTarget>() {
        let event_target: &EventTarget = resource_reference.downcast_ref().unwrap();

        if let Some(html_form_element) = event_target.dyn_ref() {
            let static_html_form_element: &'static HtmlFormElement = unsafe {
                mem::transmute::<&HtmlFormElement, &'static HtmlFormElement>(html_form_element)
            };

            Ok(static_html_form_element)
        } else {
            Err(badarg!(process).into())
        }
    } else {
        Err(badarg!(process).into())
    }
}

fn module() -> Atom {
    Atom::try_from_str("Elixir.Lumen.Web.HTMLFormElement").unwrap()
}
