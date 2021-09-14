// TODO, https://github.com/iamkun/dayjs/tree/dev/src/locale

use core::{convert::TryFrom, fmt, str::FromStr};

use crate::{Weekday, WEEKDAYS};

static EN_NAMES: &[&str] = &[
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];
static EN_ABBREVIATIONS: &[&str] = &["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
static EN_MINIMAL_ABBREVIATIONS: &[&str] = &["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];

impl FromStr for Weekday {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_en_str(s)
    }
}
impl TryFrom<&str> for Weekday {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}
impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_en_str())
    }
}

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

    pub fn from_en_str(s: &str) -> Result<Self, &'static str> {
        if let Some(i) = EN_ABBREVIATIONS.iter().position(|x| x == &s) {
            Ok(WEEKDAYS[i].to_owned())
        } else if let Some(i) = EN_NAMES.iter().position(|x| x == &s) {
            Ok(WEEKDAYS[i].to_owned())
        } else if let Some(i) = EN_MINIMAL_ABBREVIATIONS.iter().position(|x| x == &s) {
            Ok(WEEKDAYS[i].to_owned())
        } else {
            Err("unknown")
        }
    }
    pub fn to_en_str(&self) -> &str {
        self.en_abbreviation()
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

        assert_eq!(Weekday::try_from("Mon").unwrap(), Weekday::Mon);
        assert_eq!("Monday".parse::<Weekday>().unwrap(), Weekday::Mon);
        assert_eq!("Mon".parse::<Weekday>().unwrap(), Weekday::Mon);
        assert_eq!("Mo".parse::<Weekday>().unwrap(), Weekday::Mon);
        assert_eq!(Weekday::Mon.to_string(), "Mon");
    }
}
