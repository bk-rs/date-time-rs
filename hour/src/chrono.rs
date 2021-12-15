use chrono::{DateTime, NaiveDateTime, TimeZone, Timelike as _};

use crate::Hour;

impl From<NaiveDateTime> for Hour {
    fn from(dt: NaiveDateTime) -> Self {
        Self::try_from(dt.hour() as u8).unwrap()
    }
}
impl<Tz: TimeZone> From<DateTime<Tz>> for Hour {
    fn from(dt: DateTime<Tz>) -> Self {
        Self::try_from(dt.hour() as u8).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::Utc;

    #[test]
    fn simple() {
        assert_eq!(
            Hour::from("2021-08-01T00:00:00".parse::<NaiveDateTime>().unwrap()),
            Hour::C0
        );
        assert_eq!(
            Hour::from("2021-08-01T23:00:00Z".parse::<DateTime<Utc>>().unwrap()),
            Hour::C23
        );
    }
}
