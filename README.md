# duration-string

`duration-string` is a library to convert from `String` to `Duration` and vice-versa.

[![build](https://github.com/RonniSkansing/duration-string/actions/workflows/build.yaml/badge.svg)](https://github.com/RonniSkansing/duration-string/actions/workflows/build.yaml)
![Crates.io](https://img.shields.io/crates/v/duration-string.svg)

Takes a `String` such as `100ms`, `2s`, `5m 30s`, `1h10m` and converts it into a `Duration`.

Takes a `Duration` and converts it into `String`.

The `String` format is a multiply of `[0-9]+(ns|us|ms|[smhdwy])`

## Example

`String` to `Duration`:

```rust
use std::convert::TryFrom;
use duration_string::DurationString;
use std::time::Duration;

let d: Duration = DurationString::try_from(String::from("100ms")).unwrap().into();
assert_eq!(d, Duration::from_millis(100));

// Alternatively
let d: Duration = "100ms".parse::<DurationString>().unwrap().into();
assert_eq!(d, Duration::from_millis(100));
```

`Duration` to `String`:

```rust
use std::convert::TryFrom;
use duration_string::*;
use std::time::Duration;

let d: String = DurationString::from(Duration::from_millis(100)).into();
assert_eq!(d, String::from("100ms"));
```

## Serde support

You can enable _serialization/deserialization_ support by adding the feature `serde`

- Add `serde` feature

   ```toml
   duration-string = { version = "0.4.0", features = ["serde"] }
   ```

- Add derive to struct

   ```rust
   use duration_string::DurationString;
   use serde::{Deserialize, Serialize};

   #[derive(Serialize, Deserialize)]
   struct Foo {
     duration: DurationString
   }
   ```

## License

This project is licensed under the [MIT](https://opensource.org/licenses/MIT) License.

See [LICENSE](./LICENSE) file for details.
