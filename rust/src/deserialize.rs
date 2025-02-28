
use serde::{Deserialize, Serialize};
use serde_json;

// This struct will recieve the json data.
#[derive(Serialize, Deserialize, Debug)]
pub struct StockData<'a> {
    // This field is not part of the api response and will be added after
    #[serde(default)]
    pub ticker: &'a str,

    // The naming scheme of the response is ambiguous, to resolve this, field names have been renamed
    #[serde(rename = "c")]
    pub current_price: f32,

    #[serde(rename = "h")]
    pub high: f32,

    #[serde(rename = "l")]
    pub low: f32,

    #[serde(rename = "o")]
    pub opening : f32,

    #[serde(rename = "pc")]
    pub previous_close: f32,

    #[serde(rename = "t")]
    pub unix_time: i32,

    
}


pub fn deserialize<'a>(data: &'a str, ticker: &'a str)  -> serde_json::Result<StockData<'a>> {
    let mut stock_data: StockData = serde_json::from_str(data)?;  
    stock_data.ticker = ticker;

    Ok(stock_data)
}
