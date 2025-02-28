use std::env;
use reqwest::blocking::Client;


pub fn fetch_api_key() -> Result<String, env::VarError> {
    let key = "FH_Key";
    env::var(key) // fetch the api key from the enviroment variables
}


pub fn call_api(ticker: &str) -> Result<String, reqwest::Error> {

        let key = fetch_api_key()
            .unwrap_or_else(|error|format!("No Alpha Vantage API key found, please set the enviromental variable \"AV_Key\" to the API Key\nYou can do this by adding the following to the .bashrc file \nexport AV_Key=*key*\nerror:{:?}",error));



    let ticker = ticker.to_uppercase(); // tickers are always upper case
    let client = Client::new();

    let api_url = format!("https://finnhub.io/api/v1/quote?symbol={ticker}&token={key}"); //format the url to add the ticker and key

    let response = client.get(api_url)
        .send()?
        .text()?;
    Ok(response)
}
