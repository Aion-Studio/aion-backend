use chrono::{DateTime, Utc};
use prisma_client_rust::chrono::{self, Duration};
use serde::Serialize;
use serde::{Deserialize, Deserializer, Serializer};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct SerializableDateTime(Arc<Mutex<Option<DateTime<Utc>>>>);

impl Serialize for SerializableDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let locked_data = self.0.lock().unwrap();
        match *locked_data {
            Some(ref datetime) => serializer.serialize_str(&datetime.to_rfc3339()),
            None => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for SerializableDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let datetime_str: Option<String> = Option::deserialize(deserializer)?;
        let datetime_option = datetime_str
            .map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .map(|dt| dt.with_timezone(&Utc))
                    .ok()
            })
            .flatten();
        Ok(SerializableDateTime(Arc::new(Mutex::new(datetime_option))))
    }
}

#[derive(Clone, Debug)]
pub struct SerializableDuration(Duration);

impl Serialize for SerializableDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let millis = self.0.num_milliseconds();
        serializer.serialize_i64(millis)
    }
}

impl<'de> Deserialize<'de> for SerializableDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = i64::deserialize(deserializer)?;
        Ok(SerializableDuration(Duration::milliseconds(millis)))
    }
}
