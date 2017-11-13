extern crate futures;
extern crate hyper;

use std::thread;
use std::time::Duration;

use futures::future::Future;
use hyper::Body;
use hyper::header::{AccessControlAllowOrigin, ContentType};
use hyper::mime;
use hyper::server::{Http, Request, Response, Service};

struct HelloWorld;

const PHRASE: &'static str = "Hello, World!";

impl Service for HelloWorld {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        let (mut sender, body) = Body::pair();

        thread::spawn(move || loop {
            sender
                .try_send(Ok(format!("event: hello\ndata: {}\n\n", PHRASE).into()))
                .unwrap();
            thread::sleep(Duration::from_secs(1));
        });

        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentType(mime::TEXT_EVENT_STREAM))
                .with_header(AccessControlAllowOrigin::Any)
                .with_body(body),
        ))
    }
}

fn main() {
    let addr = "127.0.0.1:8888".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
    server.run().unwrap();
}
