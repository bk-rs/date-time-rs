use core::iter::Iterator;

use crate::Hour;

//
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HourIterator {
    finished: bool,
    curr_front: Option<Hour>,
    curr_back: Option<Hour>,
}
impl HourIterator {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Iterator for HourIterator {
    type Item = Hour;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let next = if let Some(w) = &self.curr_front {
            w.next()
        } else {
            Some(Hour::first())
        };
        if next == self.curr_back {
            self.finished = true;
            return None;
        }
        self.curr_front = next.to_owned();
        next
    }
}
impl DoubleEndedIterator for HourIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let prev = if let Some(w) = &self.curr_back {
            w.prev()
        } else {
            Some(Hour::last())
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
            HourIterator::new().collect::<Vec<_>>(),
            vec![
                Hour::C0,
                Hour::C1,
                Hour::C2,
                Hour::C3,
                Hour::C4,
                Hour::C5,
                Hour::C6,
                Hour::C7,
                Hour::C8,
                Hour::C9,
                Hour::C10,
                Hour::C11,
                Hour::C12,
                Hour::C13,
                Hour::C14,
                Hour::C15,
                Hour::C16,
                Hour::C17,
                Hour::C18,
                Hour::C19,
                Hour::C20,
                Hour::C21,
                Hour::C22,
                Hour::C23,
            ]
        );

        assert_eq!(
            HourIterator::new().rev().collect::<Vec<_>>(),
            vec![
                Hour::C23,
                Hour::C22,
                Hour::C21,
                Hour::C20,
                Hour::C19,
                Hour::C18,
                Hour::C17,
                Hour::C16,
                Hour::C15,
                Hour::C14,
                Hour::C13,
                Hour::C12,
                Hour::C11,
                Hour::C10,
                Hour::C9,
                Hour::C8,
                Hour::C7,
                Hour::C6,
                Hour::C5,
                Hour::C4,
                Hour::C3,
                Hour::C2,
                Hour::C1,
                Hour::C0,
            ]
        );

        // https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html#examples
        let mut iter = HourIterator::new();
        assert_eq!(iter.next(), Some(Hour::C0));
        assert_eq!(iter.next_back(), Some(Hour::C23));
        assert_eq!(iter.next_back(), Some(Hour::C22));
        assert_eq!(iter.next_back(), Some(Hour::C21));
        assert_eq!(iter.next_back(), Some(Hour::C20));
        assert_eq!(iter.next_back(), Some(Hour::C19));
        assert_eq!(iter.next_back(), Some(Hour::C18));
        assert_eq!(iter.next_back(), Some(Hour::C17));
        assert_eq!(iter.next(), Some(Hour::C1));
        assert_eq!(iter.next(), Some(Hour::C2));
        assert_eq!(iter.next(), Some(Hour::C3));
        assert_eq!(iter.next(), Some(Hour::C4));
        assert_eq!(iter.next(), Some(Hour::C5));
        assert_eq!(iter.next(), Some(Hour::C6));
        assert_eq!(iter.next(), Some(Hour::C7));
        assert_eq!(iter.next(), Some(Hour::C8));
        assert_eq!(iter.next(), Some(Hour::C9));
        assert_eq!(iter.next(), Some(Hour::C10));
        assert_eq!(iter.next(), Some(Hour::C11));
        assert_eq!(iter.next(), Some(Hour::C12));
        assert_eq!(iter.next(), Some(Hour::C13));
        assert_eq!(iter.next(), Some(Hour::C14));
        assert_eq!(iter.next(), Some(Hour::C15));
        assert_eq!(iter.next(), Some(Hour::C16));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert!(iter.finished);
        assert_eq!(iter.curr_front, Some(Hour::C16));
        assert_eq!(iter.curr_back, Some(Hour::C17));
    }
}
