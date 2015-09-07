extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::{Connection, UserAgent};

fn main() {
    // Create a client.

    // Creating an outgoing request.
    // http://lsanca-speedtest-01-a.socal.rr.com/speedtest/latency.txt # FAIL
    // http://speedtest.lax2.hostduplex.com/speedtest/latency.txt # SUCCESS
    let urls = vec!(
        "http://lsanca-speedtest-01-a.socal.rr.com/speedtest/latency.txt",
        "http://speedtest.lax2.hostduplex.com/speedtest/latency.txt",
        "http://speedtest.lax.hugeserver.com/speedtest/latency.txt",
        "http://speedtest.lax.gigenet.com/speedtest/latency.txt",
        "http://ndt.dhspeedtest.com/flash/speedtest/latency.txt",
        "http://google.com",
        "http://gamefaqs.com",
        "http://rust-lang.org",
        );
    for url in urls.iter() {
        let client = Client::new();
        let mut res = client.get(*url)
            // set a header
            .header(UserAgent("hyper/hmmweird 0.01".to_owned()))
            .header(Connection::close())
            // let 'er go!
            .send().unwrap();

        // Read the Response.
        let mut body: Vec<u8> = vec!();
        let read_res = res.read(&mut body);

        println!("Read Result for {}: {:?}", url, read_res);
    }
}
