use crate::enums::{Priority, Status};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub status: Status,
    #[serde(with = "date_format")]
    pub due_date: Option<NaiveDate>,
}

impl Task {
    pub fn new(
        name: String,
        description: Option<String>,
        priority: Priority,
        status: Status,
        due_date: Option<NaiveDate>,
    ) -> Self {
        Self {
            name,
            description,
            priority,
            status,
            due_date,
        }
    }
}

mod date_format {

    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(nd) => {
                let s = nd.format(FORMAT).to_string();
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        {
            let opt = Option::<String>::deserialize(deserializer)?;
            match opt {
                Some(s) => NaiveDate::parse_from_str(&s, FORMAT)
                    .map(Some)
                    .map_err(serde::de::Error::custom),
                None => Ok(None),
            }
        }
    }
}
