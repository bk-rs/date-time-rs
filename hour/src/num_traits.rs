use crate::{Hour, HOURS, HOUR_N_MAX, HOUR_N_MIN};

//
const HOUR_N_MIN_I64: i64 = HOUR_N_MIN as i64;
const HOUR_N_MAX_I64: i64 = HOUR_N_MAX as i64;
const HOUR_N_MIN_U64: u64 = HOUR_N_MIN as u64;
const HOUR_N_MAX_U64: u64 = HOUR_N_MAX as u64;

//
impl num_traits::FromPrimitive for Hour {
    #[inline]
    fn from_i64(n: i64) -> Option<Hour> {
        match n {
            HOUR_N_MIN_I64..=HOUR_N_MAX_I64 => Some(HOURS[(n) as usize].to_owned()),
            _ => None,
        }
    }

    #[inline]
    fn from_u64(n: u64) -> Option<Hour> {
        match n {
            HOUR_N_MIN_U64..=HOUR_N_MAX_U64 => Some(HOURS[(n) as usize].to_owned()),
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
        assert_eq!(Hour::from_usize(0), Some(Hour::C0));
        assert_eq!(Hour::from_usize(1), Some(Hour::C1));
        assert_eq!(Hour::from_usize(23), Some(Hour::C23));
        assert_eq!(Hour::from_usize(24), None);
    }
}
