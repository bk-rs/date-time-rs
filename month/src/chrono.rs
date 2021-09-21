use core::convert::TryFrom as _;

use chrono::{
    Date, DateTime, Datelike as _, Month as ChronoMonth, NaiveDate, NaiveDateTime, TimeZone,
};

use crate::Month;

pub static CHRONO_MONTHS: &[ChronoMonth] = &[
    ChronoMonth::January,
    ChronoMonth::February,
    ChronoMonth::March,
    ChronoMonth::April,
    ChronoMonth::May,
    ChronoMonth::June,
    ChronoMonth::July,
    ChronoMonth::August,
    ChronoMonth::September,
    ChronoMonth::October,
    ChronoMonth::November,
    ChronoMonth::December,
];

impl From<ChronoMonth> for Month {
    fn from(w: ChronoMonth) -> Self {
        match w {
            ChronoMonth::January => Self::Jan,
            ChronoMonth::February => Self::Feb,
            ChronoMonth::March => Self::Mar,
            ChronoMonth::April => Self::Apr,
            ChronoMonth::May => Self::May,
            ChronoMonth::June => Self::Jun,
            ChronoMonth::July => Self::Jul,
            ChronoMonth::August => Self::Aug,
            ChronoMonth::September => Self::Sep,
            ChronoMonth::October => Self::Oct,
            ChronoMonth::November => Self::Nov,
            ChronoMonth::December => Self::Dec,
        }
    }
}
impl From<NaiveDate> for Month {
    fn from(d: NaiveDate) -> Self {
        Self::try_from(d.month() as u8).unwrap()
    }
}
impl From<NaiveDateTime> for Month {
    fn from(dt: NaiveDateTime) -> Self {
        Self::try_from(dt.month() as u8).unwrap()
    }
}
impl<Tz: TimeZone> From<Date<Tz>> for Month {
    fn from(d: Date<Tz>) -> Self {
        Self::try_from(d.month() as u8).unwrap()
    }
}
impl<Tz: TimeZone> From<DateTime<Tz>> for Month {
    fn from(dt: DateTime<Tz>) -> Self {
        Self::try_from(dt.month() as u8).unwrap()
    }
}

impl From<Month> for ChronoMonth {
    fn from(w: Month) -> Self {
        match w {
            Month::Jan => Self::January,
            Month::Feb => Self::February,
            Month::Mar => Self::March,
            Month::Apr => Self::April,
            Month::May => Self::May,
            Month::Jun => Self::January,
            Month::Jul => Self::July,
            Month::Aug => Self::August,
            Month::Sep => Self::September,
            Month::Oct => Self::October,
            Month::Nov => Self::November,
            Month::Dec => Self::December,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::Utc;

    #[test]
    fn simple() {
        assert_eq!(
            Month::from("2021-08-01".parse::<NaiveDate>().unwrap()),
            Month::Aug
        );
        assert_eq!(
            Month::from("2021-08-01T00:00:00".parse::<NaiveDateTime>().unwrap()),
            Month::Aug
        );
        assert_eq!(
            Month::from(
                "2021-08-01T00:00:00Z"
                    .parse::<DateTime<Utc>>()
                    .unwrap()
                    .date()
            ),
            Month::Aug
        );
        assert_eq!(
            Month::from("2021-08-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()),
            Month::Aug
        );
    }
}
