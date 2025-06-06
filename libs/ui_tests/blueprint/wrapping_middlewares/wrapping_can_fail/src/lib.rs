use pavex::blueprint::{router::GET, Blueprint};
use pavex::f;
use pavex::middleware::Next;
use pavex::response::Response;

#[pavex::wrap(
    id = "EHANDLER_VIA_ATTRIBUTE",
    error_handler = "crate::CustomError::into_response"
)]
pub fn via_attribute<T>(_next: Next<T>) -> Result<Response, CustomError>
where
    T: IntoFuture<Output = Response>,
{
    todo!()
}

#[pavex::wrap(id = "EHANDLER_VIA_BLUEPRINT")]
// Error handler isn't specified at the macro-level, it's added
// directly when the middleware is registered against the blueprint.
pub fn via_blueprint<T>(_next: Next<T>) -> Result<Response, CustomError>
where
    T: IntoFuture<Output = Response>,
{
    todo!()
}

#[pavex::wrap(
    id = "EHANDLER_OVERRIDE_VIA_BLUEPRINT",
    error_handler = "crate::CustomError::into_response"
)]
// Error handler is specified at the macro-level, but it can be
// overridden when the middleware is registered against the blueprint.
pub fn override_in_blueprint<T>(_next: Next<T>) -> Result<Response, CustomError>
where
    T: IntoFuture<Output = Response>,
{
    todo!()
}

pub fn no_attribute<T>(_next: Next<T>) -> Result<Response, CustomError>
where
    T: IntoFuture<Output = Response>,
{
    todo!()
}

pub fn handler() -> Response {
    todo!()
}

#[derive(Debug)]
pub struct CustomError;

impl CustomError {
    pub fn into_response(&self) -> Response {
        todo!()
    }

    pub fn into_response_override(&self) -> Response {
        todo!()
    }
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();

    bp.wrap(EHANDLER_VIA_ATTRIBUTE);
    bp.wrap(EHANDLER_VIA_BLUEPRINT)
        .error_handler(f!(crate::CustomError::into_response));
    bp.wrap(EHANDLER_OVERRIDE_VIA_BLUEPRINT)
        .error_handler(f!(crate::CustomError::into_response_override));

    bp.wrap(f!(crate::no_attribute))
        .error_handler(f!(crate::CustomError::into_response));

    bp.route(GET, "/", f!(crate::handler));
    bp
}
