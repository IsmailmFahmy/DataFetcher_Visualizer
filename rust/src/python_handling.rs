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
enum SignalType {
    Start,
    Update
}

pub fn manage_graph(ticker: &str /* ,datatype : enum */, signal: SignalType) -> Result<()> {
    
    let mut child = Command::new("python3")     // Start Python script as a child process
        .arg("plot_script.py")                  
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
        "ticker/location": ticker,
        // "type": datatype,
        "signal": signal
    })
    .to_string();
    

    writeln!(stdin, "{}", init_msg)
        .expect("Failed to send json message to python");

    Ok(())
}

