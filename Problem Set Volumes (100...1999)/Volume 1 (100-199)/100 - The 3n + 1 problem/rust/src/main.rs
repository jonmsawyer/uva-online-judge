use std::io;
use std::process;

use rust::{Config, max_cycles};

fn main() -> io::Result<()> {
    loop {
        // User input string.
        let mut input = String::new();
        
        // Read user input from io::stdin and set it to `input`.
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                //print!("{} bytes read, input = {}", _n, input);
            }
            Err(error) => {
                eprintln!(">> Error: error reading from io::stdin `{}`", error);
                process::exit(1);
            },
        }
        
        // Check to see if the user wants to quit the application.
        if input.trim() == "q" || input.trim() == "quit" {
            break;
        }
        
        // Parse a new Config {} struct instance given `input` string.
        // If `input` cannot be parsed into two unsigned integers, a
        // default Config {} instance will be returned.
        let config = match Config::new(&input) {
            Ok(config) => config,
            Err(error) => {
                eprintln!(
                    ">> Error: could not parse input into a pair of unsigned integers. Reason: {}",
                    error
                );
                Config {
                    inputs: Vec::<&str>::new(),
                    i: 0,
                    j: 0,
                }
            },
        };
        
        // Get the result of the maximum cycle length between
        // integers `i` and `j`, the `result` contains the result.
        let (i, j, result) = max_cycles(config);
        println!("{} {} {}", i, j, result);
    }
    
    Ok(())
}
