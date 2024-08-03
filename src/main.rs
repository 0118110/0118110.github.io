mod markets;
mod mathematics;
mod series;
mod time;

use std::{env, error::Error, fs::File, io::Write};

use reqwest::header::AUTHORIZATION;
use serde_json::Value;

use markets::{filter_markets, Market, Markets};
use mathematics::{changes, mean_standard_deviation_ratio};
use series::Series;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let arguments: Vec<String> = env::args().collect();
    let authorization = format!("Apikey {}", &arguments[1]);
    let value = client
        .get("https://min-api.cryptocompare.com/data/top/totalvolfull?limit=100&tsym=USD")
        .header(AUTHORIZATION, &authorization)
        .send()
        .await?
        .json::<Value>()
        .await?;
    let markets = match Markets::try_from(&value) {
        Ok(value) => value,
        Err(_error) => panic!("Failed to parse markets"),
    };
    println!("Parsed {} markets", markets.get_markets().len());
    let minimum_days = arguments[2].parse::<i64>()?;
    let markets = filter_markets(markets, minimum_days);
    println!(
        "Filtered {} markets older than {} days",
        markets.get_markets().len(),
        minimum_days
    );
    let mut vector = Vec::new();
    for market in markets.get_markets() {
        let value = client
            .get(format!(
                "https://min-api.cryptocompare.com/data/v2/histoday?fsym={}&tsym=USD&limit=365",
                market.get_symbol()
            ))
            .header(AUTHORIZATION, &authorization)
            .send()
            .await?
            .json::<Value>()
            .await?;
        let series = match Series::try_from(&value) {
            Ok(value) => value,
            Err(_error) => panic!("Failed to parse series"),
        };
        let changes = changes(&series);
        let mean_standard_deviation_ratio = mean_standard_deviation_ratio(&changes);
        if mean_standard_deviation_ratio.is_nan() {
            println!(
                "{} mean_standard_deviation_ratio is NaN",
                market.get_symbol()
            );
            continue;
        }
        vector.push(Market::new(
            Some(mean_standard_deviation_ratio),
            Some(series.get_range()),
            market.get_start_date(),
            market.get_symbol().to_string(),
        ));
    }
    let mut markets = Markets::new(vector);
    let array = markets.top_k_mean_standard_deviation_ratio(arguments[3].parse::<usize>()?);
    let path = &arguments[4];
    let mut file = File::create(path)?;
    let string = serde_json::to_string(array)?;
    file.write_all(string.as_bytes())?;
    println!("Wrote {} markets to {}", array.len(), path);
    Ok(())
}
