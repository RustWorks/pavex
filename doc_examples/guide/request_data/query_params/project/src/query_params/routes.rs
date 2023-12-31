use pavex::http::StatusCode;
use pavex::request::query::QueryParams;

#[derive(serde::Deserialize)]
pub struct SearchParams {
    pub sorted: bool,
}

pub fn handler(params: &QueryParams<SearchParams>) -> StatusCode {
    if params.0.sorted {
        println!("The search is sorted");
    }
    StatusCode::OK
}
