use super::*;

#[test]
fn without_exiting_returns_true() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(&strategy::process(), |other_arc_process| {
                prop_assert!(!other_arc_process.is_exiting());
                prop_assert_eq!(
                    native(&arc_process, other_arc_process.pid_term()),
                    Ok(true.into())
                );

                Ok(())
            })
            .unwrap();
    });
}

#[test]
fn with_exiting_returns_false() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(&strategy::process(), |other_arc_process| {
                other_arc_process.exit_normal();

                prop_assert!(other_arc_process.is_exiting());
                prop_assert_eq!(
                    native(&arc_process, other_arc_process.pid_term()),
                    Ok(false.into())
                );

                Ok(())
            })
            .unwrap();
    });
}
