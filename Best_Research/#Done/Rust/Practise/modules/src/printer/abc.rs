use crate::printer::abc::a_namespace_inside_abc::public_internal::{
    public_crate_internal_fn, public_internal_fn,
};

pub fn a_function_outside_namespace() {
    public_crate_internal_fn();
    public_internal_fn();
}

pub mod a_namespace_inside_abc {
    use crate::printer::abc::a_namespace_inside_abc::private_internal::{hiddenfn, stillhidden};

    //part of the user api
    pub fn loving_apples() {
        print!("Loving Apples");
        hiddenfn();
        stillhidden();
    }

    //not part of the user api
    //only accesible inside the crate
    pub(crate) fn hate_apples() {
        print!("Loving Apples");
        hiddenfn();
        stillhidden();
    }

    pub mod public_internal {
        //this is not accessible outside the api library
        pub(crate) fn public_crate_internal_fn() {}

        //this is public accessible outside the crate
        //it would become a option for the api
        pub fn public_internal_fn() {}
    }

    mod private_internal {
        //makes no difference then just pub
        pub(crate) fn hiddenfn() {}

        pub fn stillhidden() {}
    }
}
