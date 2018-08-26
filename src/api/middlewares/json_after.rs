use iron::headers::ContentType;
use iron::AfterMiddleware;
use iron::{Iron, IronResult, Request, Response};
pub struct JSONAfterMiddleware;

impl AfterMiddleware for JSONAfterMiddleware {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(ContentType::json());
        Ok(res)
    }
}
