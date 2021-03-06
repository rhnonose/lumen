mod with_map_map1;

use proptest::prop_assert_eq;
use proptest::strategy::{Just, Strategy};
use proptest::test_runner::{Config, TestRunner};

use liblumen_alloc::badmap;

use crate::otp::maps::merge_2::native;
use crate::test::strategy;

#[test]
fn without_map_map_1_errors_badmap() {
    TestRunner::new(Config::with_source_file(file!()))
        .run(
            &strategy::process().prop_flat_map(|arc_process| {
                (
                    Just(arc_process.clone()),
                    strategy::term::is_not_map(arc_process.clone()),
                    strategy::term::is_map(arc_process.clone()),
                )
            }),
            |(arc_process, map1, map2)| {
                prop_assert_eq!(
                    native(&arc_process, map1, map2),
                    Err(badmap!(&arc_process, map1))
                );

                Ok(())
            },
        )
        .unwrap();
}
