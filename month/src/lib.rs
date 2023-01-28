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
pub use self::chrono::CHRONO_MONTHS;
#[cfg(feature = "with-serde")]
pub use self::serde::{serde_str, serde_u64};
pub use iter::MonthIterator;

//
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Month {
    Jan = 1,
    Feb = 2,
    Mar = 3,
    Apr = 4,
    May = 5,
    Jun = 6,
    Jul = 7,
    Aug = 8,
    Sep = 9,
    Oct = 10,
    Nov = 11,
    Dec = 12,
}

pub static MONTHS: &[Month] = &[
    Month::Jan,
    Month::Feb,
    Month::Mar,
    Month::Apr,
    Month::May,
    Month::Jun,
    Month::Jul,
    Month::Aug,
    Month::Sep,
    Month::Oct,
    Month::Nov,
    Month::Dec,
];
pub(crate) const MONTH_N_MIN: u8 = 1;
pub(crate) const MONTH_N_MAX: u8 = 12;

impl Month {
    pub fn first() -> Self {
        MONTHS[(MONTH_N_MIN - 1) as usize].to_owned()
    }
    pub fn last() -> Self {
        MONTHS[(MONTH_N_MAX - 1) as usize].to_owned()
    }

    pub fn next(&self) -> Option<Self> {
        if self == &Self::last() {
            None
        } else {
            Some(MONTHS[(self.to_owned() as u8 + 1 - 1) as usize].to_owned())
        }
    }
    pub fn prev(&self) -> Option<Self> {
        if self == &Self::first() {
            None
        } else {
            Some(MONTHS[(self.to_owned() as u8 - 1 - 1) as usize].to_owned())
        }
    }
}
