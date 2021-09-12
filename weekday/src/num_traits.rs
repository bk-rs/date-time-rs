use crate::{Weekday, WEEKDAYS, WEEKDAY_N_MAX, WEEKDAY_N_MIN};

const WEEKDAY_N_MIN_I64: i64 = WEEKDAY_N_MIN as i64;
const WEEKDAY_N_MAX_I64: i64 = WEEKDAY_N_MAX as i64;
const WEEKDAY_N_MIN_U64: u64 = WEEKDAY_N_MIN as u64;
const WEEKDAY_N_MAX_U64: u64 = WEEKDAY_N_MAX as u64;

impl num_traits::FromPrimitive for Weekday {
    #[inline]
    fn from_i64(n: i64) -> Option<Weekday> {
        match n {
            WEEKDAY_N_MIN_I64..=WEEKDAY_N_MAX_I64 => Some(WEEKDAYS[(n - 1) as usize].to_owned()),
            _ => None,
        }
    }

    #[inline]
    fn from_u64(n: u64) -> Option<Weekday> {
        match n {
            WEEKDAY_N_MIN_U64..=WEEKDAY_N_MAX_U64 => Some(WEEKDAYS[(n - 1) as usize].to_owned()),
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
        assert_eq!(Weekday::from_usize(0), None);
        assert_eq!(Weekday::from_usize(1), Some(Weekday::Mon));
        assert_eq!(Weekday::from_usize(7), Some(Weekday::Sun));
        assert_eq!(Weekday::from_usize(8), None);
    }
}
