use core::{
    iter::{DoubleEndedIterator, IntoIterator, Iterator},
    mem,
};

use chrono::{Duration, NaiveDate};

use crate::DateRange;

pub struct DateRangeIterator {
    since: NaiveDate,
    until: NaiveDate,
}

impl Iterator for DateRangeIterator {
    type Item = NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        // TODO
        if self.since <= self.until {
            let next = self.since + Duration::days(1);
            Some(mem::replace(&mut self.since, next))
        } else {
            None
        }
    }
}
impl DoubleEndedIterator for DateRangeIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        // TODO
        if self.since <= self.until {
            let next = self.until - Duration::days(1);
            Some(mem::replace(&mut self.until, next))
        } else {
            None
        }
    }
}

impl IntoIterator for DateRange {
    type Item = NaiveDate;

    type IntoIter = DateRangeIterator;

    fn into_iter(self) -> Self::IntoIter {
        DateRangeIterator {
            since: self.since,
            until: self.until,
        }
    }
}
impl IntoIterator for &DateRange {
    type Item = NaiveDate;

    type IntoIter = DateRangeIterator;

    fn into_iter(self) -> Self::IntoIter {
        DateRangeIterator {
            since: self.since.to_owned(),
            until: self.until.to_owned(),
        }
    }
}
