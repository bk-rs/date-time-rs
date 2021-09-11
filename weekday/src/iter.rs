use core::{
    iter::{IntoIterator, Iterator},
    mem,
};

use crate::Weekday;

#[derive(Debug, Clone, Default)]
pub struct WeekdayIterator(Option<Weekday>);
impl WeekdayIterator {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Iterator for WeekdayIterator {
    type Item = Weekday;

    fn next(&mut self) -> Option<Self::Item> {
        let next = if let Some(w) = &self.0 {
            w.next()
        } else {
            Some(Weekday::first())
        };
        mem::replace(&mut self.0, next)
    }
}
impl DoubleEndedIterator for WeekdayIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        let prev = if let Some(w) = &self.0 {
            w.prev()
        } else {
            Some(Weekday::last())
        };
        mem::replace(&mut self.0, prev)
    }
}
impl IntoIterator for Weekday {
    type Item = Weekday;

    type IntoIter = WeekdayIterator;

    fn into_iter(self) -> Self::IntoIter {
        WeekdayIterator::default()
    }
}
impl IntoIterator for &Weekday {
    type Item = Weekday;

    type IntoIter = WeekdayIterator;

    fn into_iter(self) -> Self::IntoIter {
        WeekdayIterator::default()
    }
}
