use core::iter::Iterator;

use crate::Month;

//
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MonthIterator {
    finished: bool,
    curr_front: Option<Month>,
    curr_back: Option<Month>,
}
impl MonthIterator {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Iterator for MonthIterator {
    type Item = Month;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let next = if let Some(w) = &self.curr_front {
            w.next()
        } else {
            Some(Month::first())
        };
        if next == self.curr_back {
            self.finished = true;
            return None;
        }
        self.curr_front = next.to_owned();
        next
    }
}
impl DoubleEndedIterator for MonthIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let prev = if let Some(w) = &self.curr_back {
            w.prev()
        } else {
            Some(Month::last())
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
            MonthIterator::new().collect::<Vec<_>>(),
            vec![
                Month::Jan,
                Month::Feb,
                Month::Mar,
                Month::Apr,
                Month::May,
                Month::Jun,
                Month::Jul,
                Month::Aug,
                Month::Sep,
                Month::Oct,
                Month::Nov,
                Month::Dec,
            ]
        );

        assert_eq!(
            MonthIterator::new().rev().collect::<Vec<_>>(),
            vec![
                Month::Dec,
                Month::Nov,
                Month::Oct,
                Month::Sep,
                Month::Aug,
                Month::Jul,
                Month::Jun,
                Month::May,
                Month::Apr,
                Month::Mar,
                Month::Feb,
                Month::Jan,
            ]
        );

        // https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html#examples
        let mut iter = MonthIterator::new();
        assert_eq!(iter.next(), Some(Month::Jan));
        assert_eq!(iter.next_back(), Some(Month::Dec));
        assert_eq!(iter.next_back(), Some(Month::Nov));
        assert_eq!(iter.next_back(), Some(Month::Oct));
        assert_eq!(iter.next_back(), Some(Month::Sep));
        assert_eq!(iter.next_back(), Some(Month::Aug));
        assert_eq!(iter.next_back(), Some(Month::Jul));
        assert_eq!(iter.next_back(), Some(Month::Jun));
        assert_eq!(iter.next(), Some(Month::Feb));
        assert_eq!(iter.next(), Some(Month::Mar));
        assert_eq!(iter.next(), Some(Month::Apr));
        assert_eq!(iter.next(), Some(Month::May));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert!(iter.finished);
        assert_eq!(iter.curr_front, Some(Month::May));
        assert_eq!(iter.curr_back, Some(Month::Jun));
    }
}
