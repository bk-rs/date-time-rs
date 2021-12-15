use core::fmt;

use crate::Hour;

impl fmt::Display for Hour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_en_str())
    }
}

impl Hour {
    pub fn en_name(&self) -> &str {
        match self {
            Hour::C0 => "12AM",
            Hour::C1 => "01AM",
            Hour::C2 => "02AM",
            Hour::C3 => "03AM",
            Hour::C4 => "04AM",
            Hour::C5 => "05AM",
            Hour::C6 => "06AM",
            Hour::C7 => "07AM",
            Hour::C8 => "08AM",
            Hour::C9 => "09AM",
            Hour::C10 => "10AM",
            Hour::C11 => "11AM",
            Hour::C12 => "12PM",
            Hour::C13 => "01PM",
            Hour::C14 => "02PM",
            Hour::C15 => "03PM",
            Hour::C16 => "04PM",
            Hour::C17 => "05PM",
            Hour::C18 => "06PM",
            Hour::C19 => "07PM",
            Hour::C20 => "08PM",
            Hour::C21 => "09PM",
            Hour::C22 => "10PM",
            Hour::C23 => "11PM",
        }
    }

    pub fn friendly_str(&self) -> &str {
        match self {
            Hour::C0 => "12AM - 1AM",
            Hour::C1 => "1AM - 2AM",
            Hour::C2 => "2AM - 3AM",
            Hour::C3 => "3AM - 4AM",
            Hour::C4 => "4AM - 5AM",
            Hour::C5 => "5AM - 6AM",
            Hour::C6 => "6AM - 7AM",
            Hour::C7 => "7AM - 8AM",
            Hour::C8 => "8AM - 9AM",
            Hour::C9 => "9AM - 10AM",
            Hour::C10 => "10AM - 11AM",
            Hour::C11 => "11AM - 12PM",
            Hour::C12 => "12PM - 1PM",
            Hour::C13 => "1PM - 2PM",
            Hour::C14 => "2PM - 3PM",
            Hour::C15 => "3PM - 4PM",
            Hour::C16 => "4PM - 5PM",
            Hour::C17 => "5PM - 6PM",
            Hour::C18 => "6PM - 7PM",
            Hour::C19 => "7PM - 8PM",
            Hour::C20 => "8PM - 9PM",
            Hour::C21 => "9PM - 10PM",
            Hour::C22 => "10PM - 11PM",
            Hour::C23 => "11PM - 12AM",
        }
    }

    pub fn to_en_str(&self) -> &str {
        self.en_name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn en() {
        assert_eq!(Hour::C0.en_name(), "12AM");
        assert_eq!(Hour::C0.to_string(), "12AM");
    }

    #[test]
    fn friendly_str() {
        assert_eq!(Hour::C0.friendly_str(), "12AM - 1AM");
    }
}
