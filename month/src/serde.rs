use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::Month;

//
pub mod serde_str {
    use serde::{Deserializer, Serializer};

    use super::{Month, MonthVisitor};

    pub fn serialize<S>(month: &Month, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(month.to_string().as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Month, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MonthVisitor)
    }
}

pub mod serde_u64 {
    use serde::{Deserializer, Serializer};

    use super::{Month, MonthVisitor};

    pub fn serialize<S>(month: &Month, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(month.to_owned() as u64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Month, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(MonthVisitor)
    }
}

impl Serialize for Month {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
impl<'de> Deserialize<'de> for Month {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(MonthVisitor)
    }
}

struct MonthVisitor;
impl<'de> Visitor<'de> for MonthVisitor {
    type Value = Month;

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
        m1: Month,
        #[serde(with = "crate::serde_u64")]
        m2: Month,
        #[serde(
            serialize_with = "crate::serde_str::serialize",
            deserialize_with = "crate::serde_u64::deserialize"
        )]
        m3: Month,
        m4: Month,
    }

    #[test]
    fn de() {
        let json = r#"{ "m1": "January", "m2": 2, "m3": 3, "m4": "Apr" }"#;
        assert_eq!(
            serde_json::from_str::<Foo>(json).unwrap(),
            Foo {
                m1: Month::Jan,
                m2: Month::Feb,
                m3: Month::Mar,
                m4: Month::Apr,
            }
        );

        let json = r#"{ "m1": "January", "m2": 2, "m3": 3, "m4": 4 }"#;
        assert_eq!(
            serde_json::from_str::<Foo>(json).unwrap(),
            Foo {
                m1: Month::Jan,
                m2: Month::Feb,
                m3: Month::Mar,
                m4: Month::Apr,
            }
        );
    }

    #[test]
    fn ser() {
        assert_eq!(
            serde_json::to_value(&Foo {
                m1: Month::Jan,
                m2: Month::Feb,
                m3: Month::Mar,
                m4: Month::Apr,
            })
            .unwrap(),
            json!({
                "m1": "Jan", "m2": 2, "m3": "Mar", "m4": "Apr"
            })
        );
    }
}
