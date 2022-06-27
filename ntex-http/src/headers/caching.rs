use super::Header;
use crate::{HeaderName, HeaderValue, Value};
use core::{convert::TryFrom, ops::BitOr};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Age {
    value: Duration,
}

impl Age {
    pub const FROM_ORIGIN: Age = Age {
        value: Duration::ZERO,
    };
    pub fn from_duration(value: Duration) -> Self {
        Age { value }
    }
    pub fn from_date(date: Instant) -> Self {
        Age {
            value: Instant::now().duration_since(date),
        }
    }
    pub fn get_duration(&self) -> Duration {
        self.value.clone()
    }
    pub fn set_duration(mut self, value: Duration) -> Self {
        self.value = value;
        self
    }
}

impl Header for Age {
    fn get_headername() -> HeaderName {
        crate::header::AGE
    }
    fn build(self) -> (Value, Self) {
        let header = HeaderValue::from(self.value.as_secs());
        (Value::One(header), self)
    }
}

#[doc(hidden)]
pub struct InvalidAgeValue;

impl TryFrom<HeaderValue> for Age {
    type Error = InvalidAgeValue;
    fn try_from(value: HeaderValue) -> Result<Self, Self::Error> {
        let value = value
            .to_str()
            .map_err(|_| InvalidAgeValue)?
            .trim()
            .parse::<u64>()
            .map_err(|_| InvalidAgeValue)?;
        return Ok(Age {
            value: Duration::from_secs(value),
        });
    }
}

use std::convert::TryInto;

impl TryFrom<Value> for Age {
    type Error = InvalidAgeValue;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::One(h) => return Ok(h.try_into()?),
            _ => Err(InvalidAgeValue),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct CacheFlags(u16);

impl CacheFlags {
    pub const EMPTY: CacheFlags = CacheFlags(0);
    pub const NO_CACHE: CacheFlags = CacheFlags(1);
    pub const MUST_REVALIDATE: CacheFlags = CacheFlags(1 << 1);
    pub const PROXY_REVALIDATE: CacheFlags = CacheFlags(1 << 2);
    pub const NO_STORE: CacheFlags = CacheFlags(1 << 3);
    pub const PRIVATE: CacheFlags = CacheFlags(1 << 4);
    pub const PUBLIC: CacheFlags = CacheFlags(1 << 5);
    pub const MUST_UNDERSTAND: CacheFlags = CacheFlags(1 << 6);
    pub const NO_TRANSFORM: CacheFlags = CacheFlags(1 << 7);
    pub const IMMUTABLE: CacheFlags = CacheFlags(1 << 8);
    pub const ONLY_IF_CACHED: CacheFlags = CacheFlags(1 << 9);
}
const FLAGS: [&str; 10] = [
    "no-cache",
    "must-revalidate",
    "proxy-revalidate",
    "no-store",
    "private",
    "public",
    "must-understand",
    "no-transform",
    "immutable",
    "only-if-cached",
];
impl BitOr for CacheFlags {
    type Output = CacheFlags;
    fn bitor(self, rhs: CacheFlags) -> Self::Output {
        CacheFlags(self.0 | rhs.0)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct CacheControl {
    bitflags: CacheFlags,
    max_age: Option<Duration>,
    s_maxage: Option<Duration>,
    stale_while_revalidate: Option<Duration>,
    stale_if_error: Option<Duration>,
    max_stale: Option<Duration>,
    min_fresh: Option<Duration>,
}

impl CacheControl {
    pub fn new() -> Self {
        CacheControl {
            bitflags: CacheFlags::EMPTY,
            max_age: None,
            s_maxage: None,
            stale_while_revalidate: None,
            stale_if_error: None,
            max_stale: None,
            min_fresh: None,
        }
    }
    pub fn has_flag(&self, flag: CacheFlags) -> bool {
        if flag == CacheFlags::EMPTY {
            return self.bitflags == flag;
        } else {
            return (self.bitflags.0 & flag.0) == flag.0;
        }
    }
    pub fn get_flag(&self) -> CacheFlags {
        self.bitflags.clone()
    }
    pub fn set_flag(mut self, flag: CacheFlags) -> Self {
        self.bitflags.0 = self.bitflags.0 | flag.0;
        self
    }
    pub fn remove_flag(mut self, flag: CacheFlags) -> Self {
        self.bitflags.0 = self.bitflags.0 & (!flag.0);
        self
    }
    pub fn get_max_age(&self) -> Option<Duration> {
        self.max_age.clone()
    }
    pub fn get_s_maxage(&self) -> Option<Duration> {
        self.s_maxage.clone()
    }
    pub fn get_stale_while_revalidate(&self) -> Option<Duration> {
        self.stale_while_revalidate.clone()
    }
    pub fn get_stale_if_error(&self) -> Option<Duration> {
        self.stale_if_error.clone()
    }
    pub fn get_max_stale(&self) -> Option<Duration> {
        self.max_stale.clone()
    }
    pub fn get_min_fresh(&self) -> Option<Duration> {
        self.min_fresh.clone()
    }
    pub fn set_max_age(mut self, value: Duration) -> Self {
        let _ = self.max_age.replace(value);
        self
    }
    pub fn set_s_maxage(mut self, value: Duration) -> Self {
        let _ = self.s_maxage.replace(value);
        self
    }
    pub fn set_stale_while_revalidate(mut self, value: Duration) -> Self {
        let _ = self.stale_while_revalidate.replace(value);
        self
    }
    pub fn set_stale_if_error(mut self, value: Duration) -> Self {
        let _ = self.stale_if_error.replace(value);
        self
    }
    pub fn set_max_stale(mut self, value: Duration) -> Self {
        let _ = self.max_stale.replace(value);
        self
    }
    pub fn set_min_fresh(mut self, value: Duration) -> Self {
        let _ = self.min_fresh.replace(value);
        self
    }
}

impl Header for CacheControl {
    fn get_headername() -> HeaderName {
        crate::header::CACHE_CONTROL
    }
    fn build(self) -> (Value, Self) {
        let mut vec: Vec<String> = if !self.has_flag(CacheFlags::EMPTY) {
            FLAGS
                .iter()
                .enumerate()
                .filter_map(|(n, i)| {
                    if self.has_flag(CacheFlags(1 << n)) {
                        return Some(i.to_string());
                    }
                    None
                })
                .collect::<Vec<String>>()
        } else {
            Vec::new()
        };
        if let Some(d) = self.get_max_age() {
            let value = format!("max-age={}", d.as_secs());
            vec.push(value);
        } else if let Some(d) = self.get_s_maxage() {
            let value = format!("s-maxage={}", d.as_secs());
            vec.push(value);
        } else if let Some(d) = self.get_stale_while_revalidate() {
            let value = format!("stale-while-revalidate={}", d.as_secs());
            vec.push(value);
        } else if let Some(d) = self.get_stale_if_error() {
            let value = format!("stale-if-error={}", d.as_secs());
            vec.push(value);
        } else if let Some(d) = self.get_max_stale() {
            let value = format!("max-stale={}", d.as_secs());
            vec.push(value);
        } else if let Some(d) = self.get_min_fresh() {
            let value = format!("min-fresh={}", d.as_secs());
            vec.push(value);
        }
        (
            vec.into_iter()
                .map(|s| HeaderValue::from_str(&s).expect("expect a valide Header value"))
                .collect::<Value>(),
            self,
        )
    }
}
