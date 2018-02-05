extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;

use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

#[derive(Serialize, Deserialize, Debug)]
struct IpAddress {
    origin: String,
}

fn main() {
    let address = show_ip().unwrap();
    println!("{:}", address.origin);
    ()
}

fn show_ip() -> Result<IpAddress, hyper::Error> {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());

    let uri = "http://httpbin.org/ip".parse()?;

    let work = client.get(uri).and_then(|response| {
        response
            .body()
            .concat2()
            .and_then(move |body: hyper::Chunk| {
                let address: IpAddress = serde_json::from_slice(&body)
                    .expect("Unable to parse JSON");

                Ok(address)
            })
    });

    core.run(work)
}
