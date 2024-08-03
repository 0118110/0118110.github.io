use std::cmp::min;

use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

use crate::time::{delta_days, to_datetime};

#[derive(Serialize)]
pub struct Market {
    mean_standard_deviation_ratio: Option<f64>,
    measure_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    start_date: Option<DateTime<Utc>>,
    symbol: String,
}

impl Market {
    pub fn get_mean_standard_deviation_ratio(&self) -> Option<f64> {
        self.mean_standard_deviation_ratio
    }
    pub fn get_measure_range(&self) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        self.measure_range
    }
    pub fn get_start_date(&self) -> Option<DateTime<Utc>> {
        self.start_date
    }
    pub fn get_symbol(&self) -> &String {
        &self.symbol
    }
    pub fn new(
        mean_standard_deviation_ratio: Option<f64>,
        measure_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
        start_date: Option<DateTime<Utc>>,
        symbol: String,
    ) -> Market {
        Market {
            mean_standard_deviation_ratio,
            measure_range,
            start_date,
            symbol,
        }
    }
}

pub struct Markets(Vec<Market>);

impl Markets {
    pub fn get_markets(&self) -> &Vec<Market> {
        &self.0
    }
    pub fn new(vector: Vec<Market>) -> Markets {
        Markets(vector)
    }
    fn sort_by_mean_standard_deviation_ratio(&mut self) {
        self.0.sort_by(|a, b| {
            b.get_mean_standard_deviation_ratio()
                .partial_cmp(&a.get_mean_standard_deviation_ratio())
                .unwrap()
        })
    }
    pub fn top_k_mean_standard_deviation_ratio(&mut self, k: usize) -> &[Market] {
        self.sort_by_mean_standard_deviation_ratio();
        &self.0[0..min(k, self.0.len())]
    }
}

impl TryFrom<&Value> for Markets {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if value.get("Message").unwrap().as_str().unwrap() == "Success" {
            let mut vector = Vec::new();
            for value in value.get("Data").unwrap().as_array().unwrap() {
                let map = value.get("CoinInfo").unwrap().as_object().unwrap();
                let string = map.get("AssetLaunchDate").unwrap().as_str().unwrap();
                let symbol = String::from(map.get("Name").unwrap().as_str().unwrap());
                let start_date = match to_datetime(string) {
                    Ok(datetime) => datetime,
                    Err(_error) => {
                        println!("{} AssetLaunchDate is {}", symbol, string);
                        Default::default()
                    }
                };
                vector.push(Market {
                    mean_standard_deviation_ratio: Default::default(),
                    measure_range: Default::default(),
                    start_date: Some(start_date),
                    symbol,
                })
            }
            Ok(Markets(vector))
        } else {
            Err(())
        }
    }
}

pub fn filter_markets(markets: Markets, minimum_days: i64) -> Markets {
    let end = Utc::now();
    let mut vector = Vec::new();
    for market in markets.get_markets() {
        if let Some(start) = market.get_start_date() {
            if delta_days(start, end) > minimum_days {
                vector.push(Market::new(
                    market.get_mean_standard_deviation_ratio(),
                    market.get_measure_range(),
                    Some(start),
                    market.get_symbol().to_string(),
                ))
            }
        } else {
            continue;
        }
    }
    Markets::new(vector)
}
