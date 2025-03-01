use std::{thread, time::Duration};
use deserialize::deserialize;
use anyhow::Result;


use crate::api::call_api;
use crate::db::insert_to_db;
use crate::python_handling::*;

mod deserialize;
mod db;
mod schema;
mod api;
mod python_handling;


// fn main() -> Result<()>{
//     let ticker: &str = "INTC";
//     let _ = manage_graph(ticker, SignalType::Start);
//
//     let mut loops = 1;
//
//     loop {
//
//         // let _ = process_api_to_db(ticker)?;
//         let response = call_api(ticker)
//             .unwrap_or_else(|e| format!("Error with API request: cause: {}",e ));
//         println!("got response");
//
//
//         let deserialize_data = deserialize(&response, ticker)
//             .expect("error deserializing data");
//         println!("deserialized");
//
//
//
//         let _ = insert_to_db(deserialize_data)
//             .expect("error inserting data into the database");
//         println!("inserted");
//
//
//         let _ = manage_graph(ticker, SignalType::Update);
//
//
//         loops += 1;
//         println!("loop number {}", loops);
//         thread::sleep(Duration::from_millis(36000));
//     }
// }
//
//
//
// fn process_api_to_db(ticker: &str) -> Result<()> {
//     Ok(())
// }
fn main() {

    let ticker: &str = "INTC";
        let response = call_api(ticker)
            .unwrap_or_else(|e| format!("Error with API request: cause: {}",e ));
        println!("got response");


        let deserialize_data = deserialize(&response, ticker);
        println!("{:?}", deserialize_data.unwrap());
}
