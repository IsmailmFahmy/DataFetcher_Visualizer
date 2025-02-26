use std::env;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json;

fn main() {

    let key = fetch_api_key()
        .unwrap_or_else(|error|format!("No Alpha Vantage API key found, please set the enviromental variable \"AV_Key\" to the API Key\nYou can do this by adding the following to the .bashrc file \nexport AV_Key=*key*\nerror:{:?}",error));

    let response = call_api(key,"IBM")
        .unwrap_or_else(|e| format!("Error with API request: cause: {}",e ));
    println!("{response}");


    let deserialize_data = deserialize(&response).unwrap();
    println!("{}", deserialize_data.current_price);

}

fn fetch_api_key() -> Result<String, env::VarError> {
    let key = "FH_Key";
    env::var(key)
}


fn call_api(key: String, ticker: &str) -> Result<String, reqwest::Error> {
    let ticker = ticker.to_uppercase();
    let client = Client::new();

    let api_url = format!("https://finnhub.io/api/v1/quote?symbol={ticker}&token={key}");

    let response = client.get(api_url)
        .send()?
        .text()?;
    Ok(response)
}

#[derive(Serialize, Deserialize)]
struct StockData {
    #[serde(rename = "c")]
    current_price: f32,

    #[serde(rename = "h")]
    high: f32,

    #[serde(rename = "l")]
    low: f32,

    #[serde(rename = "o")]
    opening : f32,

    #[serde(rename = "pc")]
    previous_close: f32,

    #[serde(rename = "t")]
    unix_time: u64,
}


fn deserialize(data : &str)  -> serde_json::Result<StockData> {
    let stock_data: StockData = serde_json::from_str(data)?;

    Ok(stock_data)


}
