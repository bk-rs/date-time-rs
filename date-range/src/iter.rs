use core::{
    iter::{DoubleEndedIterator, IntoIterator, Iterator},
    mem,
};

use chrono::{Duration, NaiveDate};

use crate::DateRange;

#[derive(Debug, Clone, PartialEq)]
pub struct DateRangeIterator {
    since: NaiveDate,
    until: NaiveDate,
}

// https://stackoverflow.com/questions/41679239/loop-over-date-range
impl Iterator for DateRangeIterator {
    type Item = NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.since <= self.until {
            let next_front = self.since + Duration::days(1);
            Some(mem::replace(&mut self.since, next_front))
        } else {
            None
        }
    }
}
impl DoubleEndedIterator for DateRangeIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.since <= self.until {
            let next_back = self.until - Duration::days(1);
            Some(mem::replace(&mut self.until, next_back))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(
            DateRange::new("2021-08-01".parse().unwrap(), "2021-08-05".parse().unwrap())
                .into_iter()
                .collect::<Vec<_>>(),
            vec![
                "2021-08-01".parse().unwrap(),
                "2021-08-02".parse().unwrap(),
                "2021-08-03".parse().unwrap(),
                "2021-08-04".parse().unwrap(),
                "2021-08-05".parse().unwrap(),
            ]
        );

        let date_range =
            &DateRange::new("2021-08-01".parse().unwrap(), "2021-08-05".parse().unwrap());
        assert_eq!(
            date_range.into_iter().rev().collect::<Vec<_>>(),
            vec![
                "2021-08-05".parse().unwrap(),
                "2021-08-04".parse().unwrap(),
                "2021-08-03".parse().unwrap(),
                "2021-08-02".parse().unwrap(),
                "2021-08-01".parse().unwrap(),
            ]
        );

        // https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html#examples
        let mut iter = DateRangeIterator {
            since: "2021-08-01".parse().unwrap(),
            until: "2021-08-05".parse().unwrap(),
        };
        assert_eq!(iter.next(), Some("2021-08-01".parse().unwrap()));
        assert_eq!(iter.next_back(), Some("2021-08-05".parse().unwrap()));
        assert_eq!(iter.next_back(), Some("2021-08-04".parse().unwrap()));
        assert_eq!(iter.next(), Some("2021-08-02".parse().unwrap()));
        assert_eq!(iter.next(), Some("2021-08-03".parse().unwrap()));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.since, "2021-08-04".parse().unwrap());
        assert_eq!(iter.until, "2021-08-03".parse().unwrap());
    }
}
