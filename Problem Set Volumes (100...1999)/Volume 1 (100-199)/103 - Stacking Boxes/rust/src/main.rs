#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::io;
use std::fs::File;

use rust::boxes::{Box_, Boxes};
use rust::command::Command;

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let mut _reader = stdin.lock();
    
    // Uncomment these two lines to read the commands
    // from a file called "input.txt". TODO: pass in file
    // as parameters to the execution of this program.
    //let fh = File::open("input.txt")?;
    //let mut _reader = io::BufReader::new(fh);
    
    Command::run(&mut _reader);
    
    Ok(())
}

//    let b1 = Box_ {
//        box_: vec![3, 10, 2, 4],
//    };
//    
//    let b2 = Box_ {
//        box_: vec![2, 9, 1, 3],
//    };
//    
//    let b3 = Box_ {
//        box_: vec![1, 10, 1, 3],
//    };
//    
//    let b4 = Box_ {
//        box_: vec![1, 10, 1, 3],
//    };
//    
//    println!("b1 < b2 = {}", b1 < b2);
//    println!("b1 > b2 = {}", b1 > b2);
//    println!("b2 > b3 = {}", b2 > b3);
//    println!("b2 == b3 = {}", b2 == b3);
//    println!("b3 == b4 = {}", b3 == b4);
//    println!("b3 <= b4 = {}", b3 <= b4);
//    println!("b3 >= b4 = {}", b3 >= b4);
//}
