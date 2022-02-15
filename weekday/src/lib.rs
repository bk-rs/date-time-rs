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
pub use iter::{WeekdayFromSundayIterator, WeekdayIterator};

//
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

//
pub static WEEKDAYS: &[Weekday] = &[
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
    Weekday::Sat,
    Weekday::Sun,
];

//
pub static WEEKDAYS_FROM_SUNDAY: &[Weekday] = &[
    Weekday::Sun,
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
    Weekday::Sat,
];

//
pub(crate) const WEEKDAY_N_MIN: u8 = 1;
pub(crate) const WEEKDAY_N_MAX: u8 = 7;

//
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

impl Weekday {
    pub fn first_from_sunday() -> Self {
        WEEKDAYS_FROM_SUNDAY[(WEEKDAY_N_MIN - 1) as usize].to_owned()
    }

    pub fn last_from_sunday() -> Self {
        WEEKDAYS_FROM_SUNDAY[(WEEKDAY_N_MAX - 1) as usize].to_owned()
    }

    pub fn next_from_sunday(&self) -> Option<Self> {
        if self == &Self::last_from_sunday() {
            None
        } else {
            match self {
                Self::Sun => Some(WEEKDAYS_FROM_SUNDAY[1].to_owned()),
                _ => Some(WEEKDAYS_FROM_SUNDAY[(self.to_owned() as u8 + 1) as usize].to_owned()),
            }
        }
    }

    pub fn prev_from_sunday(&self) -> Option<Self> {
        if self == &Self::first_from_sunday() {
            None
        } else {
            Some(WEEKDAYS_FROM_SUNDAY[(self.to_owned() as u8 - 1) as usize].to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        //
        assert_eq!(Weekday::first(), Weekday::Mon);
        assert_eq!(Weekday::last(), Weekday::Sun);
        assert_eq!(Weekday::Mon.next(), Some(Weekday::Tue));
        assert_eq!(Weekday::Tue.next(), Some(Weekday::Wed));
        assert_eq!(Weekday::Wed.next(), Some(Weekday::Thu));
        assert_eq!(Weekday::Thu.next(), Some(Weekday::Fri));
        assert_eq!(Weekday::Fri.next(), Some(Weekday::Sat));
        assert_eq!(Weekday::Sat.next(), Some(Weekday::Sun));
        assert_eq!(Weekday::Sun.next(), None);
        assert_eq!(Weekday::Mon.prev(), None);
        assert_eq!(Weekday::Tue.prev(), Some(Weekday::Mon));
        assert_eq!(Weekday::Wed.prev(), Some(Weekday::Tue));
        assert_eq!(Weekday::Thu.prev(), Some(Weekday::Wed));
        assert_eq!(Weekday::Fri.prev(), Some(Weekday::Thu));
        assert_eq!(Weekday::Sat.prev(), Some(Weekday::Fri));
        assert_eq!(Weekday::Sun.prev(), Some(Weekday::Sat));

        //
        assert_eq!(Weekday::first_from_sunday(), Weekday::Sun);
        assert_eq!(Weekday::last_from_sunday(), Weekday::Sat);
        assert_eq!(Weekday::Sun.next_from_sunday(), Some(Weekday::Mon));
        assert_eq!(Weekday::Mon.next_from_sunday(), Some(Weekday::Tue));
        assert_eq!(Weekday::Tue.next_from_sunday(), Some(Weekday::Wed));
        assert_eq!(Weekday::Wed.next_from_sunday(), Some(Weekday::Thu));
        assert_eq!(Weekday::Thu.next_from_sunday(), Some(Weekday::Fri));
        assert_eq!(Weekday::Fri.next_from_sunday(), Some(Weekday::Sat));
        assert_eq!(Weekday::Sat.next_from_sunday(), None);
        assert_eq!(Weekday::Sun.prev_from_sunday(), None);
        assert_eq!(Weekday::Mon.prev_from_sunday(), Some(Weekday::Sun));
        assert_eq!(Weekday::Tue.prev_from_sunday(), Some(Weekday::Mon));
        assert_eq!(Weekday::Wed.prev_from_sunday(), Some(Weekday::Tue));
        assert_eq!(Weekday::Thu.prev_from_sunday(), Some(Weekday::Wed));
        assert_eq!(Weekday::Fri.prev_from_sunday(), Some(Weekday::Thu));
        assert_eq!(Weekday::Sat.prev_from_sunday(), Some(Weekday::Fri));
    }
}
