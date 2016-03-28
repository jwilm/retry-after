extern crate chrono;
extern crate hyper;
extern crate retry_after;
extern crate httparse;

use hyper::header::Headers;
use retry_after::RetryAfter;

fn parse_delay() {
    let raw = [httparse::Header {
        name: "Retry-After",
        value: b"300",
    }];

    let headers = Headers::from_raw(&raw).unwrap();
    println!("{}", headers);

    let retry_after = headers.get::<RetryAfter>().unwrap();
    println!("{:?}", retry_after);
}

fn parse_datetime() {
    let raw = [httparse::Header {
        name: "Retry-After",
        value: b"Sun, 06 Nov 1994 08:49:37 GMT",
    }];

    let headers = Headers::from_raw(&raw).unwrap();
    println!("{}", headers);

    let retry_after = headers.get::<RetryAfter>().unwrap();
    println!("{:?}", retry_after);
}

fn main() {
    parse_delay();
    parse_datetime();
}
