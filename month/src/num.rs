use crate::{Month, MONTHS, MONTH_N_MAX, MONTH_N_MIN};

//
impl TryFrom<u8> for Month {
    type Error = &'static str;

    #[inline]
    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            MONTH_N_MIN..=MONTH_N_MAX => Ok(MONTHS[(n - 1) as usize].to_owned()),
            _ => Err("unknown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(Month::try_from(0_u8).err().unwrap(), "unknown");
        assert_eq!(Month::try_from(1_u8).unwrap(), Month::Jan);
        assert_eq!(Month::try_from(12_u8).unwrap(), Month::Dec);
        assert_eq!(Month::try_from(13_u8).err().unwrap(), "unknown");
    }
}
