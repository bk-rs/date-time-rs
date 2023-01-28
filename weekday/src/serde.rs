use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::Weekday;

//
pub mod serde_str {
    use serde::{Deserializer, Serializer};

    use super::{Weekday, WeekdayVisitor};

    pub fn serialize<S>(weekday: &Weekday, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(weekday.to_string().as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Weekday, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(WeekdayVisitor)
    }
}

pub mod serde_u64 {
    use serde::{Deserializer, Serializer};

    use super::{Weekday, WeekdayVisitor};

    pub fn serialize<S>(weekday: &Weekday, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(weekday.to_owned() as u64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Weekday, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(WeekdayVisitor)
    }
}

impl Serialize for Weekday {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
impl<'de> Deserialize<'de> for Weekday {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(WeekdayVisitor)
    }
}

struct WeekdayVisitor;
impl<'de> Visitor<'de> for WeekdayVisitor {
    type Value = Weekday;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("str or u8")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.parse::<Self::Value>().map_err(de::Error::custom)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Self::Value::try_from(v as u8).map_err(de::Error::custom)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Self::Value::try_from(v as u8).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    struct Foo {
        #[serde(with = "crate::serde_str")]
        w1: Weekday,
        #[serde(with = "crate::serde_u64")]
        w2: Weekday,
        #[serde(
            serialize_with = "crate::serde_str::serialize",
            deserialize_with = "crate::serde_u64::deserialize"
        )]
        w3: Weekday,
        w4: Weekday,
    }

    #[test]
    fn de() {
        let json = r#"{ "w1": "Monday", "w2": 2, "w3": 3, "w4": "Thu" }"#;
        assert_eq!(
            serde_json::from_str::<Foo>(json).unwrap(),
            Foo {
                w1: Weekday::Mon,
                w2: Weekday::Tue,
                w3: Weekday::Wed,
                w4: Weekday::Thu,
            }
        );

        let json = r#"{ "w1": "Monday", "w2": 2, "w3": 3, "w4": 4 }"#;
        assert_eq!(
            serde_json::from_str::<Foo>(json).unwrap(),
            Foo {
                w1: Weekday::Mon,
                w2: Weekday::Tue,
                w3: Weekday::Wed,
                w4: Weekday::Thu,
            }
        );
    }

    #[test]
    fn ser() {
        assert_eq!(
            serde_json::to_value(&Foo {
                w1: Weekday::Mon,
                w2: Weekday::Tue,
                w3: Weekday::Wed,
                w4: Weekday::Thu,
            })
            .unwrap(),
            json!({
                "w1": "Mon", "w2": 2, "w3": "Wed", "w4": "Thu"
            })
        );
    }
}
