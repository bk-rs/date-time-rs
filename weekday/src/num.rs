use core::convert::TryFrom;

use crate::{Weekday, WEEKDAYS};

impl TryFrom<u8> for Weekday {
    type Error = &'static str;

    #[inline]
    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            1..=7 => Ok(WEEKDAYS[(n - 1) as usize].to_owned()),
            _ => Err("unknown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(Weekday::try_from(1_u8).unwrap(), Weekday::Mon);
        assert_eq!(Weekday::try_from(8_u8).err().unwrap(), "unknown");
    }
}
