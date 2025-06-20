use pavex::blueprint::{from, router::GET, Blueprint};
use pavex::f;
use pavex::response::Response;

#[derive(Clone)]
#[pavex::config(key = "a")]
pub struct A<'a> {
    pub a: &'a str,
}

#[derive(Clone)]
/// One generic parameter
#[pavex::config(key = "b")]
pub struct B<T>(T);

#[derive(Clone)]
/// More than one lifetime
#[pavex::config(key = "c")]
pub struct C<'a, 'b, 'c> {
    pub a: &'a str,
    pub b: &'b str,
    pub c: &'c str,
}

#[derive(Clone)]
/// More than one generic parameter
#[pavex::config(key = "d")]
pub struct D<T, S, Z>(T, S, Z);

#[derive(Clone)]
#[allow(dead_code)]
#[pavex::config(key = "f")]
// Some static, some elided.
pub struct F<'a, 'b>(std::borrow::Cow<'a, str>, &'b str);

#[pavex::config(key = "g")]
// Decorating a re-export that doesn't point at a type.
// Case 1: function.
pub use private::not_a_type;

#[pavex::config(key = "h")]
// Decorating a re-export that doesn't point at a type.
// Case 2: enum variant.
pub use private::AnEnum::ItsVariant;

#[pavex::config(key = "i")]
// Decorating a re-export that doesn't point at a type.
// Case 3: submodule.
pub use private::a_module;

#[pavex::config(key = "j")]
// Decorating a re-export from another crate that doesn't point at a type.
// Case 1: function.
pub use dep::a_function;

#[pavex::config(key = "k")]
// Decorating a re-export from another crate that doesn't point at a type.
// Case 2: enum variant.
pub use dep::AnEnum::ItsSecondVariant;

#[pavex::config(key = "l")]
// Decorating a re-export that doesn't point at a type.
// Case 3: submodule.
pub use dep::submodule;

mod private {
    pub fn a_module() {}

    pub enum AnEnum {
        ItsVariant,
    }

    pub mod not_a_type {}
}

pub fn handler() -> Response {
    todo!()
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.import(from![crate]);
    bp.route(GET, "/", f!(crate::handler));
    bp
}
