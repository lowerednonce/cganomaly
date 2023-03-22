// use std::collections::HashMap;
use std::error::Error as stdError;
use std::fs::{File, OpenOptions};
use chrono::{Datelike, Timelike};
use std::{thread, time};
use std::io::{Write};
use serde::Deserialize;

#[derive(Deserialize)]
struct ResultJson {
    name: String,
    tickers: Vec<ListingJson>
}

#[derive(Deserialize, Debug)]
struct ListingJson {
    // the so many Option<>-s are because CoinGecko seems to like passing nulls
    _base: String,
    target: String,
    market: MarketInfoJson,
    _last: f64,
    volume: f64,
    _converted_last: ConvertedJson,
    _converted_volume: ConvertedJson,
    _trust_score: Option<String>,
    bid_ask_spread_percentage: Option<f64>,
    _timestamp: String,
    _last_traded_at: String,
    _last_fetch_at: String,
    is_anomaly: bool,
    is_stale: bool,
    _trade_url: Option<String>,
    _token_info_url: Option<String>,
    _coin_id: String,
    _target_coin_id: Option<String>,

}

#[derive(Deserialize, Debug)]
struct MarketInfoJson {
    name: String,
    _identifier: String,
    _has_trading_incentive: bool,
}

#[derive(Deserialize, Debug)]
struct ConvertedJson {
    _btc: f64,
    _eth: f64,
    _usd: f64
}

fn get_data(ticker: &String) -> Result<ResultJson, Box<dyn stdError>>{
    let tlink: String = format!("https://api.coingecko.com/api/v3/coins/{}/tickers?page=1&order=volume_dec", ticker);

    let plain_body: String = reqwest::blocking::get(tlink)?
        .text()?;

    // println!("{}", plain_body);
    
    let body: ResultJson = match serde_json::from_str::<ResultJson>(&plain_body) {
        Ok(body) => body,
        Err(e) => panic!("Failed to deserialize JSON, error: {}", e ),
    };

    println!("Successfully requested and parsed the JSON data.");

    Ok(body)

}
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn main() -> Result<(), Box<dyn stdError>>{
    let ticker = match std::env::args().nth(1) {
        Some(ticker) => ticker,
        None => panic!("Error, argument for ticker needed"),
    };

    println!("Data provided by CoinGecko"); // requirement for the free plan

    let body: ResultJson = match get_data(&ticker) {
        Ok(data) => {
            println!("working with ticker: {}", data.name);
            data
        },
        Err(e) => panic!("Error whilst getting the data: {}", e),
    };
    
    std::fs::create_dir_all(&ticker)?;
    let mut outfile = File::create(format!("./{}/{}.csv", &ticker, &ticker))?;
    // write!(outfile, "exchange,target,volume,spread,marked_anomaly,last,stale,first_digit,last_digit,volumeXspread\n")?;
    write!(outfile, "exchange,target,marked_anomaly,stale\n")?;
    for listing in &body.tickers {
        write!(outfile, "{},{},{},{}\n", listing.market.name, listing.target, listing.is_anomaly, listing.is_stale)?;
        
        // println!("{} had {} of volume", listing.market.name, listing.volume);
    }

    loop {
        let now = chrono::offset::Utc::now();
        println!("getting the data for this minute ({})", now);
        let body: ResultJson = match get_data(&ticker) {
            Ok(data) => data,
            Err(e) => panic!("Error whilst getting the data: {}", e),
        };
        for listing in &body.tickers {
            let lfname = format!("./{}/{}_{}.csv", &ticker, listing.market.name, listing.target);
            let _ = match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&lfname)
                {
                    Ok(file) => write!(&file, "date,volme,spread,last_digit,first_digit\n"),
                    Err(_) => Ok(()),
                };
            let mut file = OpenOptions::new()
                .append(true)
                .open(&lfname)?;
            let spread : f64 = match listing.bid_ask_spread_percentage {
                Some(spread) => spread,
                None => {
                    // missing value for spread
                    -1.0
                }
            };
            write!(file, "{:04}-{:02}-{:02}-{:02}-{:02},{},{},{},{}\n", 
                now.year_ce().0,
                now.month(),
                now.day(),
                now.hour(),
                now.minute(),
                listing.volume,
                spread,
                listing.volume.to_string().chars().last().unwrap(),
                listing.volume.to_string().chars().next().unwrap())?;
        }
        // wait
        thread::sleep(time::Duration::from_secs(60));
    }

    // Ok(())
}
