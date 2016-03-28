extern crate chrono;
extern crate hyper;
extern crate retry_after;

use chrono::{Duration, UTC};
use hyper::header::Headers;
use retry_after::RetryAfter;

fn retry_after_delay() {
    let mut headers = Headers::new();
    headers.set(RetryAfter::Delay(Duration::seconds(300)));

    // Should print "Retry-After: 300"
    println!("{}", headers);
}

fn retry_after_datetime() {
    let mut headers = Headers::new();
    headers.set(RetryAfter::DateTime(UTC::now() + Duration::seconds(300)));

    // Should print "Retry-After: ..."
    println!("{}", headers);
}

fn main() {
    retry_after_delay();
    retry_after_datetime();
}
