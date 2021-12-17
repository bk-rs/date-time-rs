use core::iter::Iterator;

#[cfg(feature = "with-chrono")]
use chrono::{DateTime, TimeZone};
use hour::Hour;
use weekday::Weekday;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct WeekdayAndHour(pub Weekday, pub Hour);

#[cfg(feature = "with-chrono")]
impl<Tz: TimeZone> From<DateTime<Tz>> for WeekdayAndHour {
    fn from(dt: DateTime<Tz>) -> Self {
        Self(Weekday::from(dt.clone()), Hour::from(dt))
    }
}

#[derive(Debug, Clone, Default)]
pub struct WeekdayAndHourIterator {
    curr: Option<WeekdayAndHour>,
}

impl WeekdayAndHourIterator {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Iterator for WeekdayAndHourIterator {
    type Item = WeekdayAndHour;

    fn next(&mut self) -> Option<Self::Item> {
        let new = match &self.curr {
            Some(v) => {
                let WeekdayAndHour(w, h) = v;
                match h.next() {
                    Some(h) => Some(WeekdayAndHour(w.to_owned(), h)),
                    None => w.next().map(|w| WeekdayAndHour(w, Hour::C0)),
                }
            }
            None => Some(WeekdayAndHour(Weekday::Mon, Hour::C0)),
        };
        self.curr = new.to_owned();
        new
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(WeekdayAndHourIterator::new().into_iter().count(), 7 * 24);
    }
}
