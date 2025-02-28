use std::{thread, time::Duration};
use deserialize::deserialize;
use anyhow::Result;


use crate::api::call_api;
use crate::db::insert_to_db;

mod deserialize;
mod db;
mod schema;
mod api;
mod python_handling;


fn main() -> Result<()>{
    let mut n = 0;
    let ticker: &str = "IBM";

    loop {

        let _ = process_api_to_db(ticker)?;

        // let _ = plot_graph();

        thread::sleep(Duration::from_millis(30000));
        n =n+1;
        println!("loop number {n}");
    }
}



fn process_api_to_db(ticker: &str) -> Result<()> {
        let response = call_api(ticker)
            .unwrap_or_else(|e| format!("Error with API request: cause: {}",e ));


        let deserialize_data = deserialize(&response, ticker)
            .expect("error deserializing data");



        let _ = insert_to_db(deserialize_data)
            .expect("error inserting data into the database");

    Ok(())
}
