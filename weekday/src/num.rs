use core::convert::TryFrom;

use crate::{Weekday, WEEKDAYS};

impl TryFrom<u8> for Weekday {
    type Error = &'static str;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1..=7 => Ok(WEEKDAYS[(v - 1) as usize].to_owned()),
            _ => Err("unknown"),
        }
    }
}
