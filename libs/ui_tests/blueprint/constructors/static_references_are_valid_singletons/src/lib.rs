use pavex::blueprint::{from, router::GET, Blueprint};
use pavex::f;

pub fn static_str() -> &'static str {
    todo!()
}

#[pavex::singleton]
pub fn static_u8() -> &'static u8 {
    todo!()
}

pub fn handler(_x: &'static str, _y: &'static u8) -> pavex::response::Response {
    todo!()
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.import(from![crate]);
    bp.singleton(f!(crate::static_str));
    bp.route(GET, "/", f!(crate::handler));
    bp
}
