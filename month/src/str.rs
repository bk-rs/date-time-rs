// TODO, https://github.com/iamkun/dayjs/tree/dev/src/locale

use core::{convert::TryFrom, fmt, str::FromStr};

use crate::{Month, MONTHS};

static EN_NAMES: &[&str] = &[
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
static EN_ABBREVIATIONS: &[&str] = &[
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

impl FromStr for Month {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_en_str(s)
    }
}
impl TryFrom<&str> for Month {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}
impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_en_str())
    }
}

impl Month {
    pub fn en_name(&self) -> &str {
        EN_NAMES[(self.to_owned() as u8 - 1) as usize]
    }
    pub fn en_abbreviation(&self) -> &str {
        EN_ABBREVIATIONS[(self.to_owned() as u8 - 1) as usize]
    }

    pub fn from_en_str(s: &str) -> Result<Self, &'static str> {
        if let Some(i) = EN_ABBREVIATIONS.iter().position(|x| {
            x == &s
                || if x == &"May" {
                    false
                } else {
                    format!("{}.", x) == s
                }
        }) {
            Ok(MONTHS[i].to_owned())
        } else if let Some(i) = EN_NAMES.iter().position(|x| x == &s) {
            Ok(MONTHS[i].to_owned())
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
        assert_eq!(Month::Jan.en_name(), "January");
        assert_eq!(Month::Jan.en_abbreviation(), "Jan");

        assert_eq!(Month::try_from("Jan").unwrap(), Month::Jan);
        assert_eq!("January".parse::<Month>().unwrap(), Month::Jan);
        assert_eq!("Jan".parse::<Month>().unwrap(), Month::Jan);
        assert_eq!("Jan.".parse::<Month>().unwrap(), Month::Jan);
        assert_eq!("May.".parse::<Month>().err().unwrap(), "unknown");
        assert_eq!(Month::Jan.to_string(), "Jan");
    }
}
