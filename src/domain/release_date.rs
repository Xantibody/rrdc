use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReleaseDate {
    pub title: String,
    #[serde(with = "option_date_format")]
    pub date: Option<NaiveDate>,
}

impl ReleaseDate {
    pub fn new(title: impl Into<String>, date: Option<NaiveDate>) -> Self {
        Self {
            title: title.into(),
            date,
        }
    }

    pub fn with_date(title: impl Into<String>, date: NaiveDate) -> Self {
        Self::new(title, Some(date))
    }

    pub fn undetermined(title: impl Into<String>) -> Self {
        Self::new(title, None)
    }

    pub fn is_undetermined(&self) -> bool {
        self.date.is_none()
    }
}

mod option_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";
    const UNDETERMINED: &str = "未定";

    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => serializer.serialize_str(&d.format(FORMAT).to_string()),
            None => serializer.serialize_str(UNDETERMINED),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == UNDETERMINED {
            Ok(None)
        } else {
            NaiveDate::parse_from_str(&s, FORMAT)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_with_date() {
        let release = ReleaseDate::with_date("Test Product", NaiveDate::from_ymd_opt(2024, 12, 25).unwrap());
        let json = serde_json::to_string(&release).unwrap();
        assert!(json.contains("2024-12-25"));
    }

    #[test]
    fn test_serialize_undetermined() {
        let release = ReleaseDate::undetermined("Test Product");
        let json = serde_json::to_string(&release).unwrap();
        assert!(json.contains("未定"));
    }

    #[test]
    fn test_deserialize_with_date() {
        let json = r#"{"title":"Test Product","date":"2024-12-25"}"#;
        let release: ReleaseDate = serde_json::from_str(json).unwrap();
        assert_eq!(release.date, Some(NaiveDate::from_ymd_opt(2024, 12, 25).unwrap()));
    }

    #[test]
    fn test_deserialize_undetermined() {
        let json = r#"{"title":"Test Product","date":"未定"}"#;
        let release: ReleaseDate = serde_json::from_str(json).unwrap();
        assert!(release.is_undetermined());
    }
}
