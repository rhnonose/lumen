use liblumen_alloc::erts::term::prelude::Atom;
use lumen_runtime::otp::lists;

use crate::module::NativeModule;

pub fn make_lists() -> NativeModule {
    let mut native = NativeModule::new(Atom::try_from_str("lists").unwrap());

    native.add_simple(Atom::try_from_str("keyfind").unwrap(), 3, |proc, args| {
        lists::keyfind_3::native(proc, args[0], args[1], args[2])
    });

    native.add_simple(Atom::try_from_str("member").unwrap(), 2, |proc, args| {
        lists::member_2::native(proc, args[0], args[1])
    });

    native
}
