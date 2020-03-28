//! `duration-string` is a string to duration and visa-versa lib.
//!
//! ![Crates.io](https://img.shields.io/crates/v/duration-string.svg)
//! ![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)
//!
//! Takes a string such as `100ms`, `2s`, `5m` and converts it into a `Duration`
//! Takes a duration and makes it into string.
//!
//! The string format is [0-9]+(ms|[smhdwy])
//!
//! ## Example
//!
//! String to duration
//! ```
//! use duration_string::DurationString;
//! use std::time::Duration;
//! let d: Duration = DurationString::from_string(String::from("100ms")).unwrap().into();
//! assert_eq!(d, Duration::from_millis(100));
//! ```
//! duration to string
//! ```
//! use std::convert::TryFrom;
//! use duration_string::*;
//! use std::time::Duration;
//! let d: String = DurationString::from(Duration::from_millis(100)).into();
//! assert_eq!(d, String::from("100ms"));
//! ```
//!
//! ## Serde support
//! You can enable serialize/unserialize support by adding the feature `serde`
//! - Add `serde` to the dependency
//! `duration-string = { version = "0.0.1", features = ["serde"] }`
//! - Add derive to struct
//! ```
//! # #[cfg(feature = "serde")]
//! use serde::{Deserialize, Serialize};
//! # #[cfg(feature = "serde")]
//! use serde_json;
//! use duration_string::DurationString;
//!
//! #[derive(Serialize, Deserialize)]
//! struct SerdeSupport {
//!     t: DurationString,
//! }
//! let s = SerdeSupport {
//!     t: DurationString::from_string(String::from("1m")).unwrap(),
//! };
//! assert_eq!(r#"{"t":"1m"}"#, serde_json::to_string(&s).unwrap());
//! ```

#[cfg(feature = "serde")]
use serde::de::Unexpected;
use std::convert::TryFrom;
#[cfg(feature = "serde")]
use std::fmt;
use std::fmt::Display;
#[cfg(feature = "serde")]
use std::marker::PhantomData;
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
pub struct DurationString {
    inner: Duration,
}

impl DurationString {
    pub fn new(duration: Duration) -> DurationString {
        DurationString { inner: duration }
    }

    pub fn from_string(duration: String) -> Result<Self, String> {
        DurationString::try_from(duration)
    }
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
    type Error = String;

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

        match period.parse::<u64>() {
            Ok(period) => match format.as_str() {
                "ms" => Ok(DurationString {
                    inner: Duration::from_millis(period),
                }),
                "s" => Ok(DurationString {
                    inner: Duration::from_secs(period),
                }),
                "m" => Ok(DurationString {
                    inner: Duration::from_secs(period) * MINUTE_IN_SECONDS,
                }),
                "h" => Ok(DurationString {
                    inner: Duration::from_secs(period) * HOUR_IN_SECONDS,
                }),
                "d" => Ok(DurationString {
                    inner: Duration::from_secs(period) * DAY_IN_SECONDS,
                }),
                "w" => Ok(DurationString {
                    inner: Duration::from_secs(period) * WEEK_IN_SECONDS,
                }),
                "y" => Ok(DurationString {
                    inner: Duration::from_secs(period) * YEAR_IN_SECONDS,
                }),
                _ => Err(String::from(
                    "missing TimeDuration format - must be [0-9]+(ms|[smhdwy]",
                )),
            },
            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(feature = "serde")]
struct DurationStringVisitor {
    marker: PhantomData<fn() -> DurationString>,
}

#[cfg(feature = "serde")]
impl DurationStringVisitor {
    fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for DurationStringVisitor {
    type Value = DurationString;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string")
    }

    fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match DurationString::from_string(string.to_string()) {
            Ok(d) => Ok(d),
            Err(s) => Err(serde::de::Error::invalid_value(
                Unexpected::Str(s.as_str()),
                &self,
            )),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DurationString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(DurationStringVisitor::new())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for DurationString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "serde")]
    use serde::{Deserialize, Serialize};
    #[cfg(feature = "serde")]
    use serde_json;

    #[cfg(feature = "serde")]
    #[derive(Serialize, Deserialize)]
    struct SerdeSupport {
        d: DurationString,
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_trait() {
        let s = SerdeSupport {
            d: DurationString::from_string(String::from("1m")).unwrap(),
        };
        assert_eq!(r#"{"d":"1m"}"#, serde_json::to_string(&s).unwrap());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_trait() {
        let s = r#"{"d":"2m"}"#;
        match serde_json::from_str::<SerdeSupport>(s) {
            Ok(v) => {
                assert_eq!(v.d.to_string(), "2m");
            }
            Err(err) => assert!(false, format!("failed to deserialize: {}", err)),
        }
    }

    #[test]
    fn test_string_int_overflow() {
        match DurationString::from_string(String::from("ms")) {
            Ok(_) => assert!(false, "parsing \"ms\" should fail"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    // fn test_from_string
    #[test]
    fn test_from_string() {
        let d = DurationString::from_string(String::from("100ms"));
        assert_eq!("100ms", format!("{}", d.unwrap()));
    }

    #[test]
    fn test_display_trait() {
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
