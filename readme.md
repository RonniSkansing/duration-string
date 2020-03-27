# duration-string

`duration-string` is a string to duration and visa-versa lib.

Takes a string such as `100ms`, `2s`, `5m` and converts it into a `Duration`
Takes a duration and makes it into string.

The string format is `[0-9]+(ms|[smhdwy]`

### Example

String to duration
```rust
use duration_string::DurationString;
use std::time::Duration;
let d: Duration = DurationString::from(String::from("100ms")).into();
assert_eq!(d, Duration::from_millis(100));
```
duration to string
```rust
use duration_string::DurationString;
use std::time::Duration;
let d: String = DurationString::from(Duration::from_millis(100)).into();
assert_eq!(d, String::from("100ms"));
```

### Serde support
You can enable serialize/unserialize support by adding the feature `serde_support`
- Add `serde_support` to the dependency
duration-string = { version = "0.0.1", features = ["serde_support"] }
- Add derive to struct
```rust
// use serde::{Deserialize, Serialize};
// #[derive(Serialize, Deserialize)]
// struct Foo {
//  duration: DurationString
// }
```
