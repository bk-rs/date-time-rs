use core::convert::TryFrom;

use crate::{Hour, HOURS, HOUR_N_MAX, HOUR_N_MIN};

impl TryFrom<u8> for Hour {
    type Error = &'static str;

    #[inline]
    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            HOUR_N_MIN..=HOUR_N_MAX => Ok(HOURS[(n) as usize].to_owned()),
            _ => Err("unknown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(Hour::try_from(0_u8).unwrap(), Hour::C0);
        assert_eq!(Hour::try_from(1_u8).unwrap(), Hour::C1);
        assert_eq!(Hour::try_from(23_u8).unwrap(), Hour::C23);
        assert_eq!(Hour::try_from(24_u8).err().unwrap(), "unknown");
    }
}
