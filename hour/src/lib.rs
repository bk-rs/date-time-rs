#[cfg(feature = "with-chrono")]
mod chrono;
mod iter;
mod num;
#[cfg(feature = "with-num-traits")]
mod num_traits;
mod str;

pub use iter::HourIterator;

//
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Hour {
    C0 = 0,
    C1 = 1,
    C2 = 2,
    C3 = 3,
    C4 = 4,
    C5 = 5,
    C6 = 6,
    C7 = 7,
    C8 = 8,
    C9 = 9,
    C10 = 10,
    C11 = 11,
    C12 = 12,
    C13 = 13,
    C14 = 14,
    C15 = 15,
    C16 = 16,
    C17 = 17,
    C18 = 18,
    C19 = 19,
    C20 = 20,
    C21 = 21,
    C22 = 22,
    C23 = 23,
}

pub static HOURS: &[Hour] = &[
    Hour::C0,
    Hour::C1,
    Hour::C2,
    Hour::C3,
    Hour::C4,
    Hour::C5,
    Hour::C6,
    Hour::C7,
    Hour::C8,
    Hour::C9,
    Hour::C10,
    Hour::C11,
    Hour::C12,
    Hour::C13,
    Hour::C14,
    Hour::C15,
    Hour::C16,
    Hour::C17,
    Hour::C18,
    Hour::C19,
    Hour::C20,
    Hour::C21,
    Hour::C22,
    Hour::C23,
];
pub(crate) const HOUR_N_MIN: u8 = 0;
pub(crate) const HOUR_N_MAX: u8 = 23;

impl Hour {
    pub fn first() -> Self {
        HOURS[(HOUR_N_MIN) as usize].to_owned()
    }
    pub fn last() -> Self {
        HOURS[(HOUR_N_MAX) as usize].to_owned()
    }

    pub fn next(&self) -> Option<Self> {
        if self == &Self::last() {
            None
        } else {
            Some(HOURS[(self.to_owned() as u8 + 1) as usize].to_owned())
        }
    }
    pub fn prev(&self) -> Option<Self> {
        if self == &Self::first() {
            None
        } else {
            Some(HOURS[(self.to_owned() as u8 - 1) as usize].to_owned())
        }
    }
}
