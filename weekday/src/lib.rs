// https://en.wikipedia.org/wiki/ISO_8601#Week_dates

#[cfg(feature = "with-chrono")]
mod chrono;
mod iter;
mod num;
mod str;

#[cfg(feature = "with-chrono")]
pub use self::chrono::CHRONO_WEEKDAYS;
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

pub static WEEKDAYS: &[Weekday] = &[
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
    Weekday::Sat,
    Weekday::Sun,
];

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
            Some(WEEKDAYS[(self.to_owned() as u8 + 1 - 1) as usize].to_owned())
        }
    }
    pub fn prev(&self) -> Option<Self> {
        if self == &Self::first() {
            None
        } else {
            Some(WEEKDAYS[(self.to_owned() as u8 - 1 - 1) as usize].to_owned())
        }
    }
}
