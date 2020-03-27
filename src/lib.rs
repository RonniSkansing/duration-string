//! `duration-string` is a string to duration and visa-versa lib.
//!
//! [![Crates.io][crates-badge]][crates-url]
//! [![MIT licensed][mit-badge]][mit-url]
//!
//! [crates-badge]: https://img.shields.io/crates/v/duration-string.svg
//! [mit-url]: LICENSE
//!
//! Takes a string such as `100ms`, `2s`, `5m` and converts it into a `Duration`
//! Takes a duration and makes it into string.
//!
//! The string format is [0-9]+(ms|[smhdwy
//!
//! ## Example
//!
//! String to duration
//! ```
//! use std::convert::TryFrom;
//! use duration_string::DurationString;
//! use std::time::Duration;
//! let d: Duration = DurationString::try_from(String::from("100ms")).unwrap().into();
//! assert_eq!(d, Duration::from_millis(100));
//! ```
//! duration to string
//! ```
//! use std::convert::TryFrom;
//! use duration_string::*;
//! use std::time::Duration;
//! let d: String = DurationString::try_from(Duration::from_millis(100)).unwrap().into();
//! assert_eq!(d, String::from("100ms"));
//! ```
//!
//! ## Serde support
//! You can enable serialize/unserialize support by adding the feature `serde_support`
//! - Add `serde_support` to the dependency
//! `duration-string = { version = "0.0.1", features = ["serde_support"] }`
//! - Add derive to struct
//! ```ignore
//! use serde::{Deserialize, Serialize};
//! #[derive(Serialize, Deserialize)]
//! struct Foo {
//!  duration: DurationString
//! }
//! ```

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;
use std::fmt::Display;
use std::time::Duration;

const YEAR_IN_MILLI: u128 = 31_556_926_000;
const WEEK_IN_MILLI: u128 = 604_800_000;
const DAY_IN_MILLI: u128 = 86_400_000;
const HOUR_IN_MILLI: u128 = 3_600_000;
const MINUTE_IN_MILLI: u128 = 60_000;
const SECOND_IN_MILLI: u128 = 1000;

const HOUR_IN_SECONDS: u32 = 3600;
const MINUTE_IN_SECONDS: u32 = 60;
const DAY_IN_SECONDS: u32 = 86_400;
const WEEK_IN_SECONDS: u32 = 604_800;
const YEAR_IN_SECONDS: u32 = 31_556_926;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct DurationString {
    inner: Duration,
}

impl Display for DurationString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = (*self).into();
        write!(f, "{}", s)
    }
}

impl Into<Duration> for DurationString {
    fn into(self) -> Duration {
        self.inner
    }
}

impl Into<String> for DurationString {
    fn into(self) -> String {
        let ms = self.inner.as_millis();
        println!("{}", ms);
        if ms % YEAR_IN_MILLI == 0 {
            return (ms / YEAR_IN_MILLI).to_string() + "y";
        }
        if ms % WEEK_IN_MILLI == 0 {
            return (ms / WEEK_IN_MILLI).to_string() + "w";
        }
        if ms % DAY_IN_MILLI == 0 {
            return (ms / DAY_IN_MILLI).to_string() + "d";
        }
        if ms % HOUR_IN_MILLI == 0 {
            return (ms / HOUR_IN_MILLI).to_string() + "h";
        }
        if ms % MINUTE_IN_MILLI == 0 {
            return (ms / MINUTE_IN_MILLI).to_string() + "m";
        }
        if ms % SECOND_IN_MILLI == 0 {
            return (ms / SECOND_IN_MILLI).to_string() + "s";
        }
        return ms.to_string() + "ms";
    }
}

impl From<Duration> for DurationString {
    fn from(duration: Duration) -> Self {
        DurationString { inner: duration }
    }
}

impl TryFrom<String> for DurationString {
    type Error = &'static str;

    fn try_from(duration: String) -> Result<Self, Self::Error> {
        let mut format: String = String::from("");
        let mut period: String = String::from("");

        for c in duration.chars() {
            if c.is_numeric() {
                period.push(c);
            } else {
                format.push(c);
            }
        }
        match format.as_str() {
            "ms" => Ok(DurationString {
                inner: Duration::from_millis(
                    period
                        .parse::<u64>()
                        .expect("failed to parse time duration"),
                ),
            }),
            "s" => Ok(DurationString {
                inner: Duration::from_secs(
                    period
                        .parse::<u64>()
                        .expect("failed to parse time duration"),
                ),
            }),
            "m" => Ok(DurationString {
                inner: Duration::from_secs(
                    period
                        .parse::<u64>()
                        .expect("failed to parse time duration"),
                ) * MINUTE_IN_SECONDS,
            }),
            "h" => Ok(DurationString {
                inner: Duration::from_secs(
                    period
                        .parse::<u64>()
                        .expect("failed to parse time duration"),
                ) * HOUR_IN_SECONDS,
            }),
            "d" => Ok(DurationString {
                inner: Duration::from_secs(
                    period
                        .parse::<u64>()
                        .expect("failed to parse time duration"),
                ) * DAY_IN_SECONDS,
            }),
            "w" => Ok(DurationString {
                inner: Duration::from_secs(
                    period
                        .parse::<u64>()
                        .expect("failed to parse time duration"),
                ) * WEEK_IN_SECONDS,
            }),
            "y" => Ok(DurationString {
                inner: Duration::from_secs(
                    period
                        .parse::<u64>()
                        .expect("failed to parse time duration"),
                ) * YEAR_IN_SECONDS,
            }),
            _ => Err("missing TimeDuration format - must be [0-9]+(ms|[smhdwy]"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_display_impl() {
        let d = DurationString::try_from(Duration::from_millis(100));
        assert_eq!("100ms", format!("{}", d.unwrap()));
    }

    #[test]
    fn test_from_duration() {
        let d: String = DurationString::try_from(Duration::from_millis(100))
            .unwrap()
            .into();
        assert_eq!(d, String::from("100ms"));
    }
    #[test]
    fn test_from_string_ms() {
        let d: Duration = DurationString::try_from(String::from("100ms"))
            .unwrap()
            .into();
        assert_eq!(d, Duration::from_millis(100));
    }

    #[test]
    fn test_from_string_s() {
        let d: Duration = DurationString::try_from(String::from("1s")).unwrap().into();
        assert_eq!(d, Duration::from_secs(1));
    }

    #[test]
    fn test_from_string_m() {
        let d: Duration = DurationString::try_from(String::from("1m")).unwrap().into();
        assert_eq!(d, Duration::from_secs(60));
    }

    #[test]
    fn test_from_string_h() {
        let d: Duration = DurationString::try_from(String::from("1h")).unwrap().into();
        assert_eq!(d, Duration::from_secs(3600));
    }

    #[test]
    fn test_from_string_d() {
        let d: Duration = DurationString::try_from(String::from("1d")).unwrap().into();
        assert_eq!(d, Duration::from_secs(86_400));
    }

    #[test]
    fn test_from_string_w() {
        let d: Duration = DurationString::try_from(String::from("1w")).unwrap().into();
        assert_eq!(d, Duration::from_secs(604_800));
    }

    #[test]
    fn test_from_string_y() {
        let d: Duration = DurationString::try_from(String::from("1y")).unwrap().into();
        assert_eq!(d, Duration::from_secs(31_556_926));
    }

    #[test]
    fn test_into_string_ms() {
        let d: String = DurationString::try_from(String::from("100ms"))
            .unwrap()
            .into();
        assert_eq!(d, "100ms");
    }

    #[test]
    fn test_into_string_s() {
        let d: String = DurationString::try_from(String::from("1s")).unwrap().into();
        assert_eq!(d, "1s");
    }

    #[test]
    fn test_into_string_m() {
        let d: String = DurationString::try_from(String::from("1m")).unwrap().into();
        assert_eq!(d, "1m");
    }

    #[test]
    fn test_into_string_h() {
        let d: String = DurationString::try_from(String::from("1h")).unwrap().into();
        assert_eq!(d, "1h");
    }

    #[test]
    fn test_into_string_d() {
        let d: String = DurationString::try_from(String::from("1d")).unwrap().into();
        assert_eq!(d, "1d");
    }

    #[test]
    fn test_into_string_w() {
        let d: String = DurationString::try_from(String::from("1w")).unwrap().into();
        assert_eq!(d, "1w");
    }

    #[test]
    fn test_into_string_y() {
        let d: String = DurationString::try_from(String::from("1y")).unwrap().into();
        assert_eq!(d, "1y");
    }

    #[test]
    fn test_into_string_overflow_unit() {
        let d: String = DurationString::try_from(String::from("1000ms"))
            .unwrap()
            .into();
        assert_eq!(d, "1s");

        let d: String = DurationString::try_from(String::from("60000ms"))
            .unwrap()
            .into();
        assert_eq!(d, "1m");

        let d: String = DurationString::try_from(String::from("61000ms"))
            .unwrap()
            .into();
        assert_eq!(d, "61s");
    }

    #[test]
    fn test_from_string_invalid_string() {
        match DurationString::try_from(String::from("1000x")) {
            Ok(_) => assert!(false, "should have returned an Err"),
            Err(_) => assert!(true),
        }
    }
}
