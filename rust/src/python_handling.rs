use std::process::{Command, Stdio};
use std::io::Write;
use serde_json::json;
use anyhow::Result;

// Work In Progress
// enum DataType {
//     Stock,
//     Wheather,
// }

// Use enums to add constrains :P
pub enum SignalType {
    Start,
    Update
}

pub fn manage_graph(ticker: &str /* ,datatype : DataType */, signal: SignalType) -> Result<()> {
    
    let mut child = Command::new("python3")     // Start Python script as a child process
        .current_dir("../python")            // change working directory
        .arg("./plot_script.py")                  // path to python script
        .stdin(Stdio::piped())                  // Open a pipe for sending data
        .spawn()                                // Run the command
        .expect("Failed to start Python script");



    let stdin = child.stdin
        .as_mut()
        .expect("Failed to open stdin");


    let signal = match signal {
        SignalType::Start => "start",
        SignalType::Update => "update",
    };


    // message to be sent to the python script
    let init_msg = json!({
        "ticker": ticker,
        // "type": datatype,
        "signal": signal
    })
    .to_string();
    

    writeln!(stdin, "{}", init_msg)
        .expect("Failed to send json message to python");

    Ok(())
}

