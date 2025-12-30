use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CreatedAt(DateTime<Utc>);

impl CreatedAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn from_datetime(datetime: DateTime<Utc>) -> Self {
        Self(datetime)
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn as_datetime(&self) -> &DateTime<Utc> {
        &self.0
    }

    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }

    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }

    pub fn timestamp_millis(&self) -> i64 {
        self.0.timestamp_millis()
    }
}

impl fmt::Display for CreatedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_rfc3339())
    }
}

impl From<DateTime<Utc>> for CreatedAt {
    fn from(datetime: DateTime<Utc>) -> Self {
        Self::from_datetime(datetime)
    }
}

impl From<CreatedAt> for DateTime<Utc> {
    fn from(created_at: CreatedAt) -> Self {
        created_at.0
    }
}

impl AsRef<DateTime<Utc>> for CreatedAt {
    fn as_ref(&self) -> &DateTime<Utc> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_now_creates_current_timestamp() {
        let created_at = CreatedAt::now();
        let now = Utc::now();

        // Should be within 1 second of each other
        assert!((created_at.timestamp() - now.timestamp()).abs() <= 1);
    }

    #[test]
    fn test_from_datetime() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let created_at = CreatedAt::from_datetime(datetime);

        assert_eq!(created_at.value(), datetime);
    }

    #[test]
    fn test_value_getter() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let created_at = CreatedAt::from_datetime(datetime);

        assert_eq!(created_at.value(), datetime);
    }

    #[test]
    fn test_display_formatting() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let created_at = CreatedAt::from_datetime(datetime);

        let display = format!("{}", created_at);
        assert!(display.contains("2024-01-15"));
    }

    #[test]
    fn test_from_trait() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let created_at: CreatedAt = datetime.into();

        assert_eq!(created_at.value(), datetime);
    }

    #[test]
    fn test_into_datetime() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let created_at = CreatedAt::from_datetime(datetime);
        let converted: DateTime<Utc> = created_at.into();

        assert_eq!(converted, datetime);
    }

    #[test]
    fn test_ordering() {
        let earlier =
            CreatedAt::from_datetime(Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap());
        let later = CreatedAt::from_datetime(Utc.with_ymd_and_hms(2024, 1, 15, 11, 0, 0).unwrap());

        assert!(earlier < later);
        assert!(later > earlier);
    }

    #[test]
    fn test_clone_and_copy() {
        let created_at = CreatedAt::now();
        let cloned = created_at.clone();
        let copied = created_at;

        assert_eq!(created_at, cloned);
        assert_eq!(created_at, copied);
    }

    #[test]
    fn test_timestamp_methods() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let created_at = CreatedAt::from_datetime(datetime);

        assert_eq!(created_at.timestamp(), datetime.timestamp());
        assert_eq!(created_at.timestamp_millis(), datetime.timestamp_millis());
    }
}
