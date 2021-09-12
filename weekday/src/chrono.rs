use chrono::{
    Date, DateTime, Datelike as _, NaiveDate, NaiveDateTime, TimeZone, Weekday as ChronoWeekday,
};

use crate::Weekday;

pub static CHRONO_WEEKDAYS: &[ChronoWeekday] = &[
    ChronoWeekday::Mon,
    ChronoWeekday::Tue,
    ChronoWeekday::Wed,
    ChronoWeekday::Thu,
    ChronoWeekday::Fri,
    ChronoWeekday::Sat,
    ChronoWeekday::Sun,
];

impl From<ChronoWeekday> for Weekday {
    fn from(w: ChronoWeekday) -> Self {
        match w {
            ChronoWeekday::Mon => Self::Mon,
            ChronoWeekday::Tue => Self::Tue,
            ChronoWeekday::Wed => Self::Wed,
            ChronoWeekday::Thu => Self::Thu,
            ChronoWeekday::Fri => Self::Fri,
            ChronoWeekday::Sat => Self::Sat,
            ChronoWeekday::Sun => Self::Sun,
        }
    }
}
impl From<NaiveDate> for Weekday {
    fn from(d: NaiveDate) -> Self {
        Self::from(d.weekday())
    }
}
impl From<NaiveDateTime> for Weekday {
    fn from(dt: NaiveDateTime) -> Self {
        Self::from(dt.weekday())
    }
}
impl<Tz: TimeZone> From<Date<Tz>> for Weekday {
    fn from(d: Date<Tz>) -> Self {
        Self::from(d.weekday())
    }
}
impl<Tz: TimeZone> From<DateTime<Tz>> for Weekday {
    fn from(dt: DateTime<Tz>) -> Self {
        Self::from(dt.weekday())
    }
}

impl From<Weekday> for ChronoWeekday {
    fn from(w: Weekday) -> Self {
        match w {
            Weekday::Mon => Self::Mon,
            Weekday::Tue => Self::Tue,
            Weekday::Wed => Self::Wed,
            Weekday::Thu => Self::Thu,
            Weekday::Fri => Self::Fri,
            Weekday::Sat => Self::Sat,
            Weekday::Sun => Self::Sun,
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
            Weekday::from("2021-08-01".parse::<NaiveDate>().unwrap()),
            Weekday::Sun
        );
        assert_eq!(
            Weekday::from("2021-08-01T00:00:00".parse::<NaiveDateTime>().unwrap()),
            Weekday::Sun
        );
        assert_eq!(
            Weekday::from(
                "2021-08-01T00:00:00Z"
                    .parse::<DateTime<Utc>>()
                    .unwrap()
                    .date()
            ),
            Weekday::Sun
        );
        assert_eq!(
            Weekday::from("2021-08-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()),
            Weekday::Sun
        );
    }
}
