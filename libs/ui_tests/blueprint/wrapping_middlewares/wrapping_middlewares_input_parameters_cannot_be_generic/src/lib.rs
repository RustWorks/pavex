use std::future::IntoFuture;

use pavex::blueprint::{router::GET, Blueprint};
use pavex::middleware::Next;
use pavex::response::Response;
use pavex::{f, wrap};

pub struct GenericType<V>(V);

pub fn generic_wrapping_middleware<A, T>(_next: Next<A>, _generic_input: GenericType<T>) -> Response
where
    A: IntoFuture<Output = Response>,
{
    todo!()
}

pub fn doubly_generic_wrapping_middleware<A, T, S>(
    _next: Next<A>,
    _i1: GenericType<T>,
    _i2: GenericType<S>,
) -> Response
where
    A: IntoFuture<Output = Response>,
{
    todo!()
}

pub fn triply_generic_wrapping_middleware<A, T, S, U>(
    _next: Next<A>,
    _i1: GenericType<T>,
    _i2: GenericType<S>,
    _i3: GenericType<U>,
) -> Response
where
    A: IntoFuture<Output = Response>,
{
    todo!()
}

#[wrap(id = MW_1)]
pub fn generic_wrapping_middleware1<A, T>(
    _next: Next<A>,
    _generic_input: GenericType<T>,
) -> Response
where
    A: IntoFuture<Output = Response>,
{
    todo!()
}

pub fn handler() -> pavex::response::Response {
    todo!()
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.wrap(f!(crate::generic_wrapping_middleware));
    bp.wrap(f!(crate::doubly_generic_wrapping_middleware));
    bp.wrap(f!(crate::triply_generic_wrapping_middleware));
    bp.wrap(MW_1);
    bp.route(GET, "/home", f!(crate::handler));
    bp
}
