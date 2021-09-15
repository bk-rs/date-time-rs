use core::fmt;
use std::error;

use chrono::{Duration, NaiveDate, NaiveDateTime};

mod iter;
pub use iter::DateRangeIterator;

#[derive(Debug, Clone)]
pub struct DateRange {
    since_: NaiveDate,
    until_: NaiveDate,
}
impl DateRange {
    pub fn new(since: NaiveDate, until: NaiveDate) -> Result<Self, Error> {
        if since > until {
            return Err(Error::SinceShouldLtUntil);
        }

        Ok(Self {
            since_: since,
            until_: until,
        })
    }

    pub fn since(&self) -> NaiveDate {
        self.since_.to_owned()
    }
    pub fn until(&self) -> NaiveDate {
        self.until_.to_owned()
    }

    pub fn since_datetime(&self) -> NaiveDateTime {
        self.since().and_hms(0, 0, 0)
    }
    pub fn until_datetime(&self) -> NaiveDateTime {
        self.until().and_hms(23, 59, 59)
    }

    pub fn prev(&self) -> Self {
        let prev_until = self.since() - Duration::days(1);
        let len = self.until() - self.since();
        Self {
            since_: prev_until - len,
            until_: prev_until,
        }
    }
    pub fn next(&self) -> Self {
        let next_since = self.until() + Duration::days(1);
        let len = self.until() - self.since();
        Self {
            since_: next_since,
            until_: next_since + len,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DateTimeRange {
    since_: NaiveDateTime,
    until_: NaiveDateTime,
}
impl DateTimeRange {
    pub fn new(since: NaiveDateTime, until: NaiveDateTime) -> Result<Self, Error> {
        if since > until {
            return Err(Error::SinceShouldLtUntil);
        }

        Ok(Self {
            since_: since,
            until_: until,
        })
    }

    pub fn since(&self) -> NaiveDateTime {
        self.since_.to_owned()
    }
    pub fn until(&self) -> NaiveDateTime {
        self.until_.to_owned()
    }
}
impl From<&DateRange> for DateTimeRange {
    fn from(dr: &DateRange) -> Self {
        Self {
            since_: dr.since_datetime(),
            until_: dr.until_datetime(),
        }
    }
}
impl From<DateRange> for DateTimeRange {
    fn from(dr: DateRange) -> Self {
        Self {
            since_: dr.since_datetime(),
            until_: dr.until_datetime(),
        }
    }
}

//
//
//
#[derive(Debug)]
pub enum Error {
    SinceShouldLtUntil,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn test_new() -> Result<(), Box<dyn error::Error>> {
        match DateRange::new("2021-08-02".parse().unwrap(), "2021-08-02".parse().unwrap()) {
            Ok(_) => {}
            Err(err) => {
                panic!("{}", err)
            }
        }
        match DateRange::new("2021-08-02".parse().unwrap(), "2021-08-01".parse().unwrap()) {
            Ok(_) => panic!(),
            Err(Error::SinceShouldLtUntil) => {}
        }

        Ok(())
    }

    #[test]
    fn test_since_datetime_and_until_datetime() {
        let date_range =
            DateRange::new("2021-08-09".parse().unwrap(), "2021-08-15".parse().unwrap()).unwrap();

        assert_eq!(
            date_range.since_datetime(),
            "2021-08-09T00:00:00".parse().unwrap()
        );
        assert_eq!(
            date_range.until_datetime(),
            "2021-08-15T23:59:59".parse().unwrap()
        );
    }

    #[test]
    fn test_prev_and_next() {
        let date_range =
            DateRange::new("2021-08-09".parse().unwrap(), "2021-08-15".parse().unwrap()).unwrap();

        let prev_date_range = date_range.prev();
        assert_eq!(prev_date_range.since(), "2021-08-02".parse().unwrap());
        assert_eq!(prev_date_range.until(), "2021-08-08".parse().unwrap());

        let next_date_range = date_range.next();
        assert_eq!(next_date_range.since(), "2021-08-16".parse().unwrap());
        assert_eq!(next_date_range.until(), "2021-08-22".parse().unwrap());
    }
}
