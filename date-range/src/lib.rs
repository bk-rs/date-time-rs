use core::fmt;
use std::error;

use chrono::NaiveDate;

mod iter;
pub use iter::DateRangeIterator;

#[derive(Debug, Clone)]
pub struct DateRange {
    since_: NaiveDate,
    until_: NaiveDate,
}
impl DateRange {
    pub fn new(since: NaiveDate, until: NaiveDate) -> Result<Self, DateRangeError> {
        if since > until {
            return Err(DateRangeError::SinceShouldLtUntil);
        }

        Ok(Self {
            since_: since,
            until_: until,
        })
    }

    pub fn since(&self) -> &NaiveDate {
        &self.since_
    }
    pub fn until(&self) -> &NaiveDate {
        &self.until_
    }
}

//
#[derive(Debug)]
pub enum DateRangeError {
    SinceShouldLtUntil,
}
impl fmt::Display for DateRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for DateRangeError {}
