// This is the entry point of the application. 
// It initializes the server and client, 
// and may include logic to run either 
// the client or server based on command-line arguments.

use std::env;

mod client;
mod server;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <client|server>", args[0]);
        return;
    }

    match args[1].as_str() {
        "client" => {
            if let Err(e) = client::run_client() {
                eprintln!("Error running client: {}", e);
            }
        }
        "server" => {
            if let Err(e) = server::run_server() {
                eprintln!("Error running server: {}", e);
            }
        }
        _ => eprintln!("Invalid argument. Use 'client' or 'server'."),
    }
}