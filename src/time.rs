use std::error::Error;

use chrono::{DateTime, NaiveDate, Utc};

pub fn delta_days(start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
    (end - start).num_days()
}

pub fn to_datetime(date: &str) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let naivedate = NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(
        naivedate.and_hms_opt(0, 0, 0).unwrap(),
        Utc,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_returned_error() {
        assert!(to_datetime("0000-00-00").is_err());
    }

    #[test]
    fn made_delta_days() {
        let actual = delta_days(
            to_datetime("1970-01-01").unwrap(),
            to_datetime("1970-01-02").unwrap(),
        );
        assert_eq!(actual, 1);
    }
}
