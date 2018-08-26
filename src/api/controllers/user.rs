use iron::{Iron, IronResult, Request, Response};
pub fn create(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with("Hello World"))
}

pub fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with("Hello World"))
}

pub fn view(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with("Hello World"))
}
