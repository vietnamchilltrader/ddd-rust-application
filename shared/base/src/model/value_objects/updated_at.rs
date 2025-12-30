use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UpdatedAt(DateTime<Utc>);

impl UpdatedAt {
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

impl fmt::Display for UpdatedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_rfc3339())
    }
}

impl From<DateTime<Utc>> for UpdatedAt {
    fn from(datetime: DateTime<Utc>) -> Self {
        Self::from_datetime(datetime)
    }
}

impl From<UpdatedAt> for DateTime<Utc> {
    fn from(updated_at: UpdatedAt) -> Self {
        updated_at.0
    }
}

impl AsRef<DateTime<Utc>> for UpdatedAt {
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
        let updated_at = UpdatedAt::now();
        let now = Utc::now();

        assert!((updated_at.timestamp() - now.timestamp()).abs() <= 1);
    }

    #[test]
    fn test_from_datetime() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let updated_at = UpdatedAt::from_datetime(datetime);

        assert_eq!(updated_at.value(), datetime);
    }

    #[test]
    fn test_value_getter() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let updated_at = UpdatedAt::from_datetime(datetime);

        assert_eq!(updated_at.value(), datetime);
    }

    #[test]
    fn test_display_formatting() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let updated_at = UpdatedAt::from_datetime(datetime);

        let display = format!("{}", updated_at);
        assert!(display.contains("2024-01-15"));
    }

    #[test]
    fn test_to_rfc3339() {
        let datetime = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let updated_at = UpdatedAt::from_datetime(datetime);

        assert_eq!(updated_at.to_rfc3339(), "2023-01-01T12:00:00+00:00");
    }

    #[test]
    fn test_from_trait() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let updated_at: UpdatedAt = datetime.into();

        assert_eq!(updated_at.value(), datetime);
    }

    #[test]
    fn test_into_datetime() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let updated_at = UpdatedAt::from_datetime(datetime);
        let converted: DateTime<Utc> = updated_at.into();

        assert_eq!(converted, datetime);
    }

    #[test]
    fn test_ordering() {
        let earlier =
            UpdatedAt::from_datetime(Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap());
        let later = UpdatedAt::from_datetime(Utc.with_ymd_and_hms(2024, 1, 15, 11, 0, 0).unwrap());

        assert!(earlier < later);
        assert!(later > earlier);
    }

    #[test]
    fn test_clone_and_copy() {
        let updated_at = UpdatedAt::now();
        let cloned = updated_at.clone();
        let copied = updated_at;

        assert_eq!(updated_at, cloned);
        assert_eq!(updated_at, copied);
    }

    #[test]
    fn test_timestamp_methods() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let updated_at = UpdatedAt::from_datetime(datetime);

        assert_eq!(updated_at.timestamp(), datetime.timestamp());
        assert_eq!(updated_at.timestamp_millis(), datetime.timestamp_millis());
    }

    #[test]
    fn test_as_ref() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let updated_at = UpdatedAt::from_datetime(datetime);
        let datetime_ref: &DateTime<Utc> = updated_at.as_ref();

        assert_eq!(datetime_ref, &datetime);
    }
}
