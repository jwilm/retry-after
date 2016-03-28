retry-after
===========

[![Build Status](https://travis-ci.org/jwilm/retry-after.svg?branch=master)](https://travis-ci.org/jwilm/retry-after)

Retry-After header for [Hyper][]. Implemented according to [RFC7231-7.1.3][].

## Usage

For more in-depth examples, please see the [examples](examples) directory.

```rust
extern crate chrono;
extern crate retry_after;

use chrono::{Duration, UTC};
use retry_after::RetryAfter;

fn main() {
    // Create a RetryAfter::Delay header
    let retry_after_delay = RetryAfter::Delay(Duration::seconds(300));

    // Create a RetryAfter::DateTime header
    let retry_after_dt = RetryAfter::DateTime(UTC::now() + Duration::seconds(300));
}
```

[Hyper]: https://github.com/hyperium/hyper
[RFC7231-7.1.3]: http://tools.ietf.org/html/rfc7231#section-7.1.3
