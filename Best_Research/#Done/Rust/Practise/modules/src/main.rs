use crate::printer::abc::{
    self, a_function_outside_namespace,
    a_namespace_inside_abc::{
        loving_apples,
        public_internal::{self, public_crate_internal_fn, public_internal_fn},
    },
};

mod printer;

//not flatten
//`abc` is explicit
// use crate::printer::abc::{
//     self, a_function_outside_namespace, a_namespace_inside_abc::loving_apples,
// };

//flatten
//hiding the namespace abc
//`abc` remains an internal implementation/detail module used only to organize
//use crate::printer::{a_function_outside_namespace, a_namespace_inside_abc::loving_apples, abc};

fn main() {
    //mod
    printer::hello_world();
    printer::abc::a_namespace_inside_abc::loving_apples();
    printer::abc::a_function_outside_namespace();

    //use crate
    abc::a_function_outside_namespace();
    abc::a_namespace_inside_abc::loving_apples();
    loving_apples();
    a_function_outside_namespace();

    //internal_public mod
    public_internal::public_crate_internal_fn();
    public_internal::public_internal_fn();
    public_crate_internal_fn();
    public_internal_fn();
}
