#[cfg(feature = "with-chrono")]
mod chrono;
mod iter;
mod num;
#[cfg(feature = "with-num-traits")]
mod num_traits;
#[cfg(feature = "with-serde")]
mod serde;
mod str;

#[cfg(feature = "with-chrono")]
pub use self::chrono::CHRONO_WEEKDAYS;
#[cfg(feature = "with-serde")]
pub use self::serde::{serde_str, serde_u64};
pub use iter::WeekdayIterator;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Copy, Clone)]
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
pub(crate) const WEEKDAY_N_MIN: u8 = 1;
pub(crate) const WEEKDAY_N_MAX: u8 = 7;

impl Weekday {
    pub fn first() -> Self {
        WEEKDAYS[(WEEKDAY_N_MIN - 1) as usize].to_owned()
    }
    pub fn last() -> Self {
        WEEKDAYS[(WEEKDAY_N_MAX - 1) as usize].to_owned()
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
