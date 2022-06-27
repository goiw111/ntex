use std::fmt;

const MQ: u16 = 1000;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Quality {
    Value(u16),
    Default,
}

impl Quality {
    ///most preferred 1
    pub const MP: Quality = Quality::Default;
    ///least preferred 0.001
    pub const LP: Quality = Quality::Value(1);
    ///not acceptable 0
    pub const NA: Quality = Quality::Value(0);

    pub fn from_f32(f: f32) -> Option<Quality> {
        if f < 0.0 {
            return None;
        }
        if (f * 1000.0) as u16 > MQ {
            return None;
        }
        Some(Quality::Value((f * 1000.0) as u16))
    }
}

impl Default for Quality {
    fn default() -> Self {
        Quality::Default
    }
}

impl fmt::Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quality::Value(q) => {
                if *q == 0 {
                    return f.write_str("q=0");
                } else if *q > MQ {
                    return Err(fmt::Error);
                } else {
                    return write!(f, "q={:-}", *q as f32 / 1000.0);
                }
            }
            _ => Ok(()),
        }
    }
}
