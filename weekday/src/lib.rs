// https://en.wikipedia.org/wiki/ISO_8601#Week_dates

use core::iter::{IntoIterator, Iterator};

use chrono::Weekday as ChronoWeekday;

pub enum Weekday {
    Mon = 1,
    Tue = 2,
    Wed = 3,
    Thu = 4,
    Fri = 5,
    Sat = 6,
    Sun = 7,
}

#[derive(Default)]
pub struct WeekdayIterator(Option<Weekday>);
impl Iterator for WeekdayIterator {
    type Item = Weekday;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
impl DoubleEndedIterator for WeekdayIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
impl IntoIterator for Weekday {
    type Item = Weekday;

    type IntoIter = WeekdayIterator;

    fn into_iter(self) -> Self::IntoIter {
        WeekdayIterator::default()
    }
}

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
