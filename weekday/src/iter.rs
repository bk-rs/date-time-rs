use core::iter::Iterator;

use crate::Weekday;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct WeekdayIterator {
    finished: bool,
    curr_front: Option<Weekday>,
    curr_back: Option<Weekday>,
}
impl WeekdayIterator {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Iterator for WeekdayIterator {
    type Item = Weekday;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let next = if let Some(w) = &self.curr_front {
            w.next()
        } else {
            Some(Weekday::first())
        };
        if next == self.curr_back {
            self.finished = true;
            return None;
        }
        self.curr_front = next.to_owned();
        next
    }
}
impl DoubleEndedIterator for WeekdayIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let prev = if let Some(w) = &self.curr_back {
            w.prev()
        } else {
            Some(Weekday::last())
        };
        if prev == self.curr_front {
            self.finished = true;
            return None;
        }
        self.curr_back = prev.to_owned();
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(
            WeekdayIterator::new().into_iter().collect::<Vec<_>>(),
            vec![
                Weekday::Mon,
                Weekday::Tue,
                Weekday::Wed,
                Weekday::Thu,
                Weekday::Fri,
                Weekday::Sat,
                Weekday::Sun,
            ]
        );

        assert_eq!(
            WeekdayIterator::new().into_iter().rev().collect::<Vec<_>>(),
            vec![
                Weekday::Sun,
                Weekday::Sat,
                Weekday::Fri,
                Weekday::Thu,
                Weekday::Wed,
                Weekday::Tue,
                Weekday::Mon,
            ]
        );

        let mut iter = WeekdayIterator::new();
        assert_eq!(iter.next(), Some(Weekday::Mon));
        assert_eq!(iter.next_back(), Some(Weekday::Sun));
        assert_eq!(iter.next_back(), Some(Weekday::Sat));
        assert_eq!(iter.next(), Some(Weekday::Tue));
        assert_eq!(iter.next(), Some(Weekday::Wed));
        assert_eq!(iter.next(), Some(Weekday::Thu));
        assert_eq!(iter.next(), Some(Weekday::Fri));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert!(iter.finished);
        assert_eq!(iter.curr_front, Some(Weekday::Fri));
        assert_eq!(iter.curr_back, Some(Weekday::Sat));
    }
}
