use chrono::{DateTime, Utc};
use serde_json::Value;

pub struct Data {
    close: f64,
    time: DateTime<Utc>,
}

impl Data {
    pub fn get_close(&self) -> f64 {
        self.close
    }
    pub fn get_time(&self) -> DateTime<Utc> {
        self.time
    }
}

pub struct Series(Vec<Data>);

impl Series {
    pub fn get_range(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        (self.0[0].get_time(), self.0[&self.0.len() - 1].get_time())
    }

    pub fn get_series(&self) -> &Vec<Data> {
        &self.0
    }
}

impl TryFrom<&Value> for Series {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if value.get("Response").unwrap().as_str().unwrap() == "Success" {
            let mut series = Vec::new();
            let values = value
                .get("Data")
                .unwrap()
                .get("Data")
                .unwrap()
                .as_array()
                .unwrap();
            for value in values {
                series.push(Data {
                    close: value.get("close").unwrap().as_f64().unwrap(),
                    time: DateTime::from_timestamp(value.get("time").unwrap().as_i64().unwrap(), 0)
                        .unwrap(),
                });
            }
            Ok(Series(series))
        } else {
            Err(())
        }
    }
}
