// https://en.wikipedia.org/wiki/ISO_8601#Week_dates

use core::convert::TryFrom;

#[cfg(feature = "with-chrono")]
mod chrono;

mod iter;
pub use iter::WeekdayIterator;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone)]
#[repr(u8)]
pub enum Weekday {
    Mon = 1,
    Tue = 2,
    Wed = 3,
    Thu = 4,
    Fri = 5,
    Sat = 6,
    Sun = 7,
}
impl TryFrom<u8> for Weekday {
    type Error = &'static str;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Mon),
            2 => Ok(Self::Tue),
            3 => Ok(Self::Wed),
            4 => Ok(Self::Thu),
            5 => Ok(Self::Fri),
            6 => Ok(Self::Sat),
            7 => Ok(Self::Sun),
            _ => Err("unknown"),
        }
    }
}
impl Weekday {
    pub fn short_str(&self) -> &str {
        match self {
            Weekday::Mon => "Mon",
            Weekday::Tue => "Tue",
            Weekday::Wed => "Wed",
            Weekday::Thu => "Thu",
            Weekday::Fri => "Fri",
            Weekday::Sat => "Sat",
            Weekday::Sun => "Sun",
        }
    }
}

impl Weekday {
    pub fn first() -> Self {
        Self::Mon
    }
    pub fn last() -> Self {
        Self::Sun
    }

    pub fn next(&self) -> Option<Self> {
        if self == &Self::last() {
            None
        } else {
            Some(Self::try_from((self.to_owned() as u8) + 1).unwrap())
        }
    }
    pub fn prev(&self) -> Option<Self> {
        if self == &Self::first() {
            None
        } else {
            Some(Self::try_from((self.to_owned() as u8) - 1).unwrap())
        }
    }
}
