use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

const SERIALIZE_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
const DESERIALIZE_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S %z";

pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    if let Some(date) = date {
        let datetime: DateTime<Utc> = Utc.from_utc_datetime(date);
        let formatted = datetime.format(SERIALIZE_DATE_FORMAT).to_string();
        serializer.serialize_str(&formatted)
    } else {
        serializer.serialize_none()
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;

    if let Some(value) = value {
        let datetime = DateTime::parse_from_str(&value, DESERIALIZE_DATE_FORMAT);
        match datetime {
            Ok(datetime) => {
                Ok(Some(datetime.naive_utc()))
            }
            Err(error) => {
                tracing::error!("时间格式错误:{}", error);
                Err(serde::de::Error::custom(error.to_string()))
            }
        }
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod testing {
    #[test]
    pub fn test_date_format() {
        let str = "2024-01-27 15:52:17 +0200";
        let datetime = chrono::DateTime::parse_and_remainder(str.trim(), super::DESERIALIZE_DATE_FORMAT);
        match datetime {
            Ok((datetime, value)) => {
                let naive = datetime.naive_utc();
                println!("naive:{}", naive);
            }
            Err(error) => {
                println!("error:{}", error);
            }
        }
    }
}