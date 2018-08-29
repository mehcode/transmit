#![feature(
    uniform_paths,
    pin,
    async_await,
    futures_api,
    await_macro,
    arbitrary_self_types,
    underscore_imports
)]
#![warn(rust_2018_idioms, future_incompatible)]

use failure::Error;
use hyper::Client;
use std::str::from_utf8;

// General purpose future prelude (used in `.map`)
use futures::prelude::*;

// Needed to interact directly with the futures01 stream (in `.concat2()`)
use futures01::Stream;

// facade over std::await! that allows to await futures from both futures01 and futures03
use tokio::await;

fn main() {
    tokio::run_async(get("http://httpbin.org/get").map(|res| {
        println!("{:?}", res);
    }));
}

async fn get(url: &str) -> Result<(), Error> {
    let client = Client::new();

    let response = await!(client.get(url.parse()?))?;

    let bytes = await!(response.into_body().concat2())?;
    let text = from_utf8(&*bytes)?;

    println!("{}", text);

    Ok(())
}
