use anyhow::Result;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::schema::stockdata;
use crate::deserialize::StockData;


#[derive(Insertable)]
#[diesel(table_name = stockdata)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct NewStockData<'a> {
    ticker: &'a str,
    unixtime: i32,
    currentprice: f32,
    high: f32,
    low: f32,
    opening: f32,
    pclose: f32,
     
}


pub fn establish_connection() -> SqliteConnection {
    
    dotenv().ok(); // ensures the .env file exists

    let database_url = env::var("DATABASE_URL") // Gets database url from the .env file
        .expect("DATABASE_URL enviromental variable must be set in .env");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn insert_to_db(data: StockData) -> Result<()> {
    use crate::schema::*;


    let mut connection = establish_connection(); //establish a connection to the db

    let stock_entry = NewStockData {
        ticker: data.ticker,
        unixtime: data.unix_time,
        currentprice: data.current_price,
        high: data.high,
        low: data.low,
        opening: data.opening,
        pclose: data.previous_close,

    };

    diesel::insert_into(stockdata::table)   // insert to table "stockdata"
        .values(&stock_entry)               // insert the values in stock entry
        .execute(&mut connection)           // use the given connection
        .expect("Error saving new video");

    Ok(())
}
