mod caching;
mod content_negotiation;

pub use self::caching::{Age, CacheControl, CacheFlags};
pub use self::content_negotiation::Quality;

use crate::{HeaderName, Value};

pub trait Header {
    fn get_headername() -> HeaderName;
    fn build(self) -> (Value, Self);
}
