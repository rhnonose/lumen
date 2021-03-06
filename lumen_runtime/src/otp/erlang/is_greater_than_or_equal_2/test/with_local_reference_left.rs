use super::*;

#[test]
fn with_number_or_atom_returns_true() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::local_reference(arc_process.clone()),
                    strategy::term::number_or_atom(arc_process),
                ),
                |(left, right)| {
                    prop_assert_eq!(native(left, right), true.into());

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_greater_local_reference_right_returns_true() {
    is_greater_than_or_equal(|_, process| process.reference(0).unwrap(), true);
}

#[test]
fn with_same_value_local_reference_right_returns_true() {
    is_greater_than_or_equal(|_, process| process.reference(1).unwrap(), true);
}

#[test]
fn with_greater_local_reference_right_returns_false() {
    is_greater_than_or_equal(|_, process| process.reference(2).unwrap(), false);
}

#[test]
fn with_function_port_pid_tuple_map_list_or_bitstring_returns_false() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::local_reference(arc_process.clone()),
                    strategy::term::function_port_pid_tuple_map_list_or_bitstring(
                        arc_process.clone(),
                    ),
                ),
                |(left, right)| {
                    prop_assert_eq!(native(left, right), false.into());

                    Ok(())
                },
            )
            .unwrap();
    });
}

fn is_greater_than_or_equal<R>(right: R, expected: bool)
where
    R: FnOnce(Term, &Process) -> Term,
{
    super::is_greater_than_or_equal(|process| process.reference(1).unwrap(), right, expected);
}
