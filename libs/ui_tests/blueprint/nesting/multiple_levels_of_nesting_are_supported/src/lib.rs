use pavex::blueprint::{constructor::Lifecycle, router::GET, Blueprint};
use pavex::f;
use pavex::http::StatusCode;

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.constructor(f!(crate::first), Lifecycle::RequestScoped);
    bp.prefix("/first").nest(first_bp());
    bp
}

fn first_bp() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.constructor(f!(crate::second), Lifecycle::RequestScoped);
    bp.prefix("/second").nest(second_bp());
    bp
}

fn second_bp() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.constructor(f!(crate::third), Lifecycle::RequestScoped);
    bp.prefix("/third").nest(third_bp());
    bp
}

fn third_bp() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.route(GET, "/home", f!(crate::handler));
    bp
}

pub fn first() -> u16 {
    todo!()
}

pub fn second(_x: u16) -> u32 {
    todo!()
}

pub fn third(_x: u32) -> String {
    todo!()
}

pub fn handler(_x: String) -> StatusCode {
    todo!()
}
