use std::io;

use rust::{Config, max_cycles};

fn main() -> io::Result<()> {
    loop {
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                //print!("{} bytes read, input = {}", n, input);
            }
            Err(error) => eprintln!("error: {}", error),
        }
        
        let config = match Config::new(&input) {
            Ok(config) => config,
            Err(_) => {
                eprintln!(">> Error: could not parse input into a pair of integers.");
                Config {
                    inputs: Vec::<&str>::new(),
                    i: 0,
                    j: 0,
                }
            },
        };
        
        let (i, j, result) = max_cycles(config);
        println!("{} {} {}", i, j, result);
    }
    
    //Ok(())
}
