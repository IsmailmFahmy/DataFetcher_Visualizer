use std::env;
use reqwest::blocking::Client;

fn main() {

    let key = fetch_api_key()
        .unwrap_or_else(|error|format!("No Alpha Vantage API key found, please set the enviromental variable \"AV_Key\" to the API Key\nYou can do this by adding the following to the .bashrc file \nexport AV_Key=*key*\nerror:{:?}",error));

    let response = call_api(key,"IBM")
        .unwrap_or_else(|e| format!("Error with API request: cause: {}",e ));
    println!("{response}");



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

