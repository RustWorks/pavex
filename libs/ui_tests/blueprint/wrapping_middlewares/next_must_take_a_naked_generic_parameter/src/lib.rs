use std::future::{IntoFuture, Ready};

use pavex::blueprint::{router::GET, Blueprint};
use pavex::middleware::Next;
use pavex::response::Response;
use pavex::{f, wrap};

pub struct Custom<T>(T);

impl<T> IntoFuture for Custom<T> {
    type Output = Response;
    type IntoFuture = Ready<Response>;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}

pub fn mw<T>(_next: Next<Custom<T>>) -> Response
where
    T: IntoFuture<Output = Response>,
{
    todo!()
}

#[wrap]
pub fn mw1<T>(_next: Next<Custom<T>>) -> Response
where
    T: IntoFuture<Output = Response>,
{
    todo!()
}

pub fn handler() -> Response {
    todo!()
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.wrap(f!(crate::mw));
    bp.wrap(MW_1);
    bp.route(GET, "/home", f!(crate::handler));
    bp
}
