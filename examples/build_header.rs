use std::time::{Duration, SystemTime};

use retry_after::{self, RetryAfter};
use http::header::HeaderMap;

fn retry_after_delay() {
    let mut headers = HeaderMap::new();
    headers.insert(retry_after::HEADER_NAME, RetryAfter::Delay(Duration::from_secs(300)).into());

    println!("{:?}", headers);
}

fn retry_after_datetime() {
    let mut headers = HeaderMap::new();
    let retry_after = RetryAfter::DateTime(SystemTime::now() + Duration::from_secs(300)).into();
    headers.insert(retry_after::HEADER_NAME, retry_after);

    println!("{:?}", headers);
}

fn main() {
    retry_after_delay();
    retry_after_datetime();
}
