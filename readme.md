# duration-string

`duration-string` is a string to duration and visa-versa lib.

![Crates.io](https://img.shields.io/crates/v/duration-string.svg)
![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)

Takes a string such as `100ms`, `2s`, `5m 30s`, `1h10m` and converts it into a `Duration`.

Takes a duration and makes it into string.

The string format is multiples of `[0-9]+(ns|us|ms|[smhdwy])`

### Example

String to duration
```rust
use std::convert::TryFrom;
use duration_string::DurationString;
use std::time::Duration;
let d: Duration = DurationString::try_from(String::from("100ms")).unwrap().into();
assert_eq!(d, Duration::from_millis(100));
// Alternatively:
let d: Duration = "100ms".parse::<DurationString>().unwrap().into();
assert_eq!(d, Duration::from_millis(100));
```
duration to string
```rust
use std::convert::TryFrom;
use duration_string::*;
use std::time::Duration;
let d: String = DurationString::from(Duration::from_millis(100)).into();
assert_eq!(d, String::from("100ms"));
```

### Serde support
You can enable serialize/unserialize support by adding the feature `serde`
- Add `serde` to the dependency
`duration-string = { version = "0.0.1", features = ["serde"] }`
- Add derive to struct
```rust
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct Foo {
 duration: DurationString
}
```

License: MIT
