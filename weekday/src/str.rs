// https://github.com/iamkun/dayjs/tree/dev/src/locale

static EN_NAMES: &[&'static str] = &[
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];
static EN_ABBREVIATIONS: &[&'static str] = &["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
static EN_MINIMAL_ABBREVIATIONS: &[&'static str] = &["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];

use crate::Weekday;

impl Weekday {
    pub fn en_name(&self) -> &str {
        EN_NAMES[(self.to_owned() as u8 - 1) as usize]
    }
    pub fn en_abbreviation(&self) -> &str {
        EN_ABBREVIATIONS[(self.to_owned() as u8 - 1) as usize]
    }
    pub fn en_minimal_abbreviation(&self) -> &str {
        EN_MINIMAL_ABBREVIATIONS[(self.to_owned() as u8 - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn en() {
        assert_eq!(Weekday::Mon.en_name(), "Monday");
        assert_eq!(Weekday::Mon.en_abbreviation(), "Mon");
        assert_eq!(Weekday::Mon.en_minimal_abbreviation(), "Mo");
    }
}
