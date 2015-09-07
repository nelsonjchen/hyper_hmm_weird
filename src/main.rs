extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::{Connection, UserAgent};

fn main() {
    // Create a client.
    let client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get("http://lsanca-speedtest-01-a.socal.rr.com/")
        // set a header
        .header(UserAgent("hyper/hmmweird 0.01".to_owned()))
        .header(Connection::close())
        // let 'er go!
        .send().unwrap();

    // Read the Response.
    let mut body: Vec<u8> = vec!();
    let read_res = res.read(&mut body);

    println!("Read Result: {:?}", read_res);
}
