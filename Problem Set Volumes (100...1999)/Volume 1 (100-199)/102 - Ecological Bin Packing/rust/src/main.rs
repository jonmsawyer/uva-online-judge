#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::io;
use std::fs::File;

use rust::Bins;

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let mut _reader = stdin.lock();
    
    // Uncomment these two lines to read the commands
    // from a file called "input.txt". TODO: pass in file
    // as parameters to the execution of this program.
    //let fh = File::open("input.txt")?;
    //let mut _reader = io::BufReader::new(fh);
    
    Bins::run(&mut _reader);
    
    Ok(())
}
