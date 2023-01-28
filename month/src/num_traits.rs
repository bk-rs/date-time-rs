use crate::{Month, MONTHS, MONTH_N_MAX, MONTH_N_MIN};

//
const MONTH_N_MIN_I64: i64 = MONTH_N_MIN as i64;
const MONTH_N_MAX_I64: i64 = MONTH_N_MAX as i64;
const MONTH_N_MIN_U64: u64 = MONTH_N_MIN as u64;
const MONTH_N_MAX_U64: u64 = MONTH_N_MAX as u64;

//
impl num_traits::FromPrimitive for Month {
    #[inline]
    fn from_i64(n: i64) -> Option<Month> {
        match n {
            MONTH_N_MIN_I64..=MONTH_N_MAX_I64 => Some(MONTHS[(n - 1) as usize].to_owned()),
            _ => None,
        }
    }

    #[inline]
    fn from_u64(n: u64) -> Option<Month> {
        match n {
            MONTH_N_MIN_U64..=MONTH_N_MAX_U64 => Some(MONTHS[(n - 1) as usize].to_owned()),
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
        assert_eq!(Month::from_usize(0), None);
        assert_eq!(Month::from_usize(1), Some(Month::Jan));
        assert_eq!(Month::from_usize(12), Some(Month::Dec));
        assert_eq!(Month::from_usize(13), None);
    }
}
