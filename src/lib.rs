//! A Hello World example application for working with Gotham.

extern crate iron;

use iron::prelude::*;
use iron::status;

const HELLO_WORLD: &'static str = "Hello World!";

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, HELLO_WORLD)))
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn run() {
    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}
