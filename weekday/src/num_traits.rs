use crate::{Weekday, WEEKDAYS};

impl num_traits::FromPrimitive for Weekday {
    #[inline]
    fn from_i64(n: i64) -> Option<Weekday> {
        match n {
            1..=7 => Some(WEEKDAYS[(n - 1) as usize].to_owned()),
            _ => None,
        }
    }

    #[inline]
    fn from_u64(n: u64) -> Option<Weekday> {
        match n {
            1..=7 => Some(WEEKDAYS[(n - 1) as usize].to_owned()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use num_traits::FromPrimitive as _;

    #[test]
    fn simple() {
        assert_eq!(Weekday::from_usize(1), Some(Weekday::Mon));
        assert_eq!(Weekday::from_usize(8), None);
    }
}
