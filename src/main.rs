extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

fn main() {
    // Create a client.
    let mut client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get("http://lsanca-speedtest-01-a.socal.rr.com/speedtest/latency.txt")
        // set a header
        .header(Connection::close())
        // let 'er go!
        .send().unwrap();

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
}
