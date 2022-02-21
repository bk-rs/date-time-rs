use core::fmt;
use std::error;

use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};

mod iter;
pub use iter::DateRangeIterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DateRange {
    since_: NaiveDate,
    until_: NaiveDate,
}

impl Default for DateRange {
    fn default() -> Self {
        Self {
            since_: NaiveDate::from_ymd(1970, 1, 1),
            until_: NaiveDate::from_ymd(1970, 1, 1),
        }
    }
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
    pub fn from_timestamp(
        since: i64,
        mut until: i64,
        configuration: impl Into<Option<DateRangeFromTimestampConfiguration>>,
    ) -> Result<Self, Error> {
        let configuration = configuration.into().unwrap_or_default();

        if since > until {
            return Err(Error::SinceShouldLtUntil);
        }

        if configuration.now_gteq_until {
            let now = Utc::now().timestamp();
            if until > now {
                until = now;

                if since > until {
                    return Err(Error::SinceShouldLtUntil);
                }
            }
        }

        let since = NaiveDateTime::from_timestamp(since, 0);
        let until = NaiveDateTime::from_timestamp(until, 0);

        let mut since = since.date();
        let until = until.date();

        if (until - since).num_days() > configuration.max_interval.num_days() {
            since = until - configuration.max_interval;
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

#[derive(Debug, Copy, Clone)]
pub struct DateRangeFromTimestampConfiguration {
    pub now_gteq_until: bool,
    pub max_interval: Duration,
}
impl Default for DateRangeFromTimestampConfiguration {
    fn default() -> Self {
        Self {
            now_gteq_until: true,
            max_interval: Duration::days(2 * 365), // 2 years
        }
    }
}
impl DateRangeFromTimestampConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}

//
//
//
#[derive(Debug, Copy, Clone)]
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
    fn test_date_range_default() {
        assert_eq!(
            DateRange::default(),
            DateRange::new("1970-01-01".parse().unwrap(), "1970-01-01".parse().unwrap()).unwrap()
        )
    }

    #[test]
    fn test_date_range_new() -> Result<(), Box<dyn error::Error>> {
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
    fn test_date_range_from_timestamp() {
        let now = Utc::now();

        //
        let date_range = DateRange::from_timestamp(
            (now - Duration::days(7)).timestamp(),
            (now + Duration::days(3)).timestamp(),
            None,
        )
        .unwrap();
        assert_eq!(
            date_range.since(),
            now.naive_utc().date() - Duration::days(7)
        );
        assert_eq!(date_range.until(), now.naive_utc().date());

        //
        let mut configuration = DateRangeFromTimestampConfiguration::new();
        configuration.now_gteq_until = false;
        let date_range = DateRange::from_timestamp(
            (now - Duration::days(7)).timestamp(),
            (now + Duration::days(3)).timestamp(),
            configuration,
        )
        .unwrap();
        assert_eq!(
            date_range.since(),
            now.naive_utc().date() - Duration::days(7)
        );
        assert_eq!(
            date_range.until(),
            now.naive_utc().date() + Duration::days(3)
        );

        //
        let date_range = DateRange::from_timestamp(
            (now - Duration::days(10)).timestamp(),
            now.timestamp(),
            Some(DateRangeFromTimestampConfiguration {
                now_gteq_until: true,
                max_interval: Duration::days(7),
            }),
        )
        .unwrap();
        assert_eq!(
            date_range.since(),
            now.naive_utc().date() - Duration::days(7)
        );
        assert_eq!(date_range.until(), now.naive_utc().date());
    }

    #[test]
    fn test_date_range_since_datetime_and_until_datetime() {
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
    fn test_date_range_prev_and_next() {
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
