extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

fn main() {
    // Create a client.
    let mut client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get("http://lsanca-speedtest-01-a.socal.rr.com/speedtest/random4000x4000.jpg")
        // set a header
        .header(Connection::close())
        // let 'er go!
        .send().unwrap();

    // Read the Response.
    let mut body: Vec<u8>= vec!();
    res.read(&mut body).unwrap();

    println!("Response: {:?}", body);
}
