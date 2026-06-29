use crate::printer::abc::a_namespace_inside_abc::public_internal;
use crate::printer::abc::a_namespace_inside_abc::public_internal::public_crate_internal_fn;
use crate::printer::abc::a_namespace_inside_abc::public_internal::public_internal_fn;

pub mod abc;

//example how we can simplify a very long path to a single word
pub use abc::a_namespace_inside_abc::public_internal::public_double_internal::halleluja as woot;

//flattening
//hiding the namespace abc
//`abc` remains an internal implementation/detail module used only to organize
//pub use abc::*;

pub fn hello_world() {
    println!("Hello World!");
    public_internal::public_crate_internal_fn();
    public_internal::public_internal_fn();
    public_crate_internal_fn();
    public_internal_fn();
}
