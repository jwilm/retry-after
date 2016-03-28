//! A `Retry-After` header implementation for Hyper
//!
//! This crate's repo is located at https://github.com/jwilm/retry-after.
//!
//! # Examples
//!
//! ```
//! extern crate chrono;
//! extern crate retry_after;
//!
//! use chrono::{Duration, UTC};
//! use retry_after::RetryAfter;
//!
//! # fn main() {
//! // Create a RetryAfter::Delay header
//! let retry_after_delay = RetryAfter::Delay(Duration::seconds(300));
//!
//! // Create a RetryAfter::DateTime header
//! let retry_after_dt = RetryAfter::DateTime(UTC::now() + Duration::seconds(300));
//! # }
//! ```
//!
//! For more examples, please see the _examples_ directory at the crate root.
//!
extern crate chrono;
extern crate hyper;

use std::fmt;

use chrono::{UTC, TimeZone, DateTime};
use hyper::header::{Header, HeaderFormat};

/// Retry-After header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-7.1.3)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RetryAfter {
    /// Retry after this duration has elapsed
    ///
    /// This can be coupled with a response time header to produce a DateTime.
    Delay(chrono::Duration),

    /// Retry after the given DateTime
    DateTime(chrono::DateTime<UTC>),
}

impl Header for RetryAfter {
    fn header_name() -> &'static str {
        "Retry-After"
    }

    fn parse_header(raw: &[Vec<u8>]) -> hyper::Result<RetryAfter> {
        if raw.len() == 0 {
            return Err(hyper::Error::Header);
        }

        let line = &raw[0];
        let utf8_str = match ::std::str::from_utf8(line) {
            Ok(utf8_str) => utf8_str,
            Err(_) => return Err(hyper::Error::Header),
        };

        // Try and parse it as an integer, first.
        if let Ok(seconds) = utf8_str.parse::<i64>() {
            return Ok(RetryAfter::Delay(chrono::Duration::seconds(seconds)));
        }

        // Now, try and parse it as a DateTime.
        if let Ok(datetime) = parse_http_date(utf8_str) {
            return Ok(RetryAfter::DateTime(datetime));
        }

        Err(hyper::Error::Header)
    }
}

impl HeaderFormat for RetryAfter {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RetryAfter::Delay(ref duration) => {
                write!(f, "{}", duration.num_seconds())
            },
            RetryAfter::DateTime(ref datetime) => {
                // According to RFC7231, the sender of an HTTP-date must use the RFC1123 format.
                // http://tools.ietf.org/html/rfc7231#section-7.1.1.1
                write!(f, "{}", datetime.format(RFC1123_FMT).to_string())
            }
        }
    }
}

static RFC850_FMT: &'static str =  "%A, %d-%b-%y %T GMT";
static RFC1123_FMT: &'static str = "%a, %d %b %Y %T GMT";
static ASCTIME_FMT: &'static str = "%a %b %e %T %Y";

/// Parse an HTTP-date
///
/// HTTP/1.1 servers must return HTTP-dates using RFC1123 format for Retry-After. For compatibility
/// with HTTP/1.0 servers, RFC850 and ASCTIME formats are supported as well.
fn parse_http_date(raw: &str) -> Result<DateTime<UTC>, &'static str> {
    if let Ok(dt) = UTC.datetime_from_str(raw, RFC1123_FMT) {
        Ok(dt)
    } else if let Ok(dt) = UTC.datetime_from_str(raw, RFC850_FMT) {
        Ok(dt)
    } else if let Ok(dt) = UTC.datetime_from_str(raw, ASCTIME_FMT) {
        Ok(dt)
    } else {
        Err("Could not parse.")
    }
}

#[cfg(test)]
mod tests {
    extern crate httparse;

    use hyper::header::{Header, Headers};
    use chrono::{self, UTC, TimeZone, Duration};

    use super::{RFC850_FMT, RFC1123_FMT, ASCTIME_FMT};
    use super::RetryAfter;

    macro_rules! test_parse_format {
        ($name:ident, $fmt:ident, $dt_str:expr) => {
            #[test]
            fn $name() {
                let dt = UTC.ymd(1994, 11, 6).and_hms(8, 49, 37);

                // Check that the format is what we expect
                assert_eq!(dt.format($fmt).to_string(), $dt_str);

                // Check that it parses correctly
                assert_eq!(Ok(dt), UTC.datetime_from_str($dt_str, $fmt));
            }
        }
    }

    test_parse_format!(parse_rfc1123, RFC1123_FMT, "Sun, 06 Nov 1994 08:49:37 GMT");
    test_parse_format!(parse_rfc850,  RFC850_FMT,  "Sunday, 06-Nov-94 08:49:37 GMT");
    test_parse_format!(parse_asctime, ASCTIME_FMT, "Sun Nov  6 08:49:37 1994");


    #[test]
    fn header_name_regression() {
        assert_eq!(RetryAfter::header_name(), "Retry-After");
    }

    #[test]
    fn parse_delay() {
        let delay_raw = [b"1234".to_vec()];
        let retry_after = RetryAfter::parse_header(&delay_raw).unwrap();

        assert_eq!(RetryAfter::Delay(chrono::Duration::seconds(1234)), retry_after);
    }

    macro_rules! test_retry_after_datetime {
        ($name:ident, $bytes:expr) => {
            #[test]
            fn $name() {
                let raw = [$bytes.to_vec()];
                let dt = UTC.ymd(1994, 11, 6).and_hms(8, 49, 37);

                let retry_after = RetryAfter::parse_header(&raw).expect("parse_header ok");
                assert_eq!(RetryAfter::DateTime(dt), retry_after);
            }
        }
    }

    test_retry_after_datetime!(header_parse_rfc1123, b"Sun, 06 Nov 1994 08:49:37 GMT");
    test_retry_after_datetime!(header_parse_rfc850, b"Sunday, 06-Nov-94 08:49:37 GMT");
    test_retry_after_datetime!(header_parse_asctime, b"Sun Nov  6 08:49:37 1994");

    #[test]
    fn hyper_headers_from_raw_delay() {
        let raw = [httparse::Header {
            name: "Retry-After",
            value: b"300",
        }];

        let headers = Headers::from_raw(&raw).unwrap();
        let retry_after = headers.get::<RetryAfter>().unwrap();
        assert_eq!(retry_after, &RetryAfter::Delay(Duration::seconds(300)));
    }

    #[test]
    fn hyper_headers_from_raw_datetime() {
        let raw = [httparse::Header {
            name: "Retry-After",
            value: b"Sun, 06 Nov 1994 08:49:37 GMT",
        }];

        let headers = Headers::from_raw(&raw).unwrap();
        let retry_after = headers.get::<RetryAfter>().unwrap();

        let expected = UTC.ymd(1994, 11, 6).and_hms(8, 49, 37);
        assert_eq!(retry_after, &RetryAfter::DateTime(expected));
    }
}
