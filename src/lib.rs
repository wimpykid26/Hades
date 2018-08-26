//! A Hello World example application for working with Gotham.

extern crate iron;
extern crate mount;
#[macro_use]
extern crate router;
use iron::prelude::Chain;
use iron::status;
use iron::Iron;
use mount::Mount;

pub mod api;
/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn run() {
    let mut chain = Chain::new(api::controllers::user::update);
    chain.link_after(api::middlewares::json_after::JSONAfterMiddleware);
    let user_router = router! {
        user_index: get "/" => api::controllers::user::index,
        user_view: get "/:id" => api::controllers::user::view,
        user_create: post "/" => api::controllers::user::create,
        user_update: post "/:id" => chain,
    };

    let mut mount = Mount::new();
    mount.mount("/user", user_router);
    Iron::new(mount).http("localhost:3000").unwrap();
}
