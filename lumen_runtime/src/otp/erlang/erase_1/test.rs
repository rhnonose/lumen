use proptest::prop_assert_eq;
use proptest::test_runner::{Config, TestRunner};

use liblumen_alloc::erts::term::prelude::Atom;

use crate::otp::erlang::erase_1::native;
use crate::scheduler::with_process_arc;
use crate::test::strategy;

#[test]
fn without_key_returns_undefined() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(&strategy::term(arc_process.clone()), |key| {
                prop_assert_eq!(native(&arc_process, key), Atom::str_to_term("undefined"));

                Ok(())
            })
            .unwrap();
    });
}

#[test]
fn with_key_returns_value_and_removes_key_from_dictionary() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term(arc_process.clone()),
                    strategy::term(arc_process.clone()),
                ),
                |(key, value)| {
                    arc_process.put(key, value).unwrap();

                    prop_assert_eq!(arc_process.get_value_from_key(key), value);

                    prop_assert_eq!(native(&arc_process, key), value);

                    prop_assert_eq!(
                        arc_process.get_value_from_key(key),
                        Atom::str_to_term("undefined")
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}
