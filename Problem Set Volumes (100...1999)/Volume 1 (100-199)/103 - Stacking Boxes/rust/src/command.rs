//! `command` module.
//!
//! Author: Jonathan Sawyer <jonmsawyer[at]gmail.com>
//!
//! Date: 2020-06-19
//!
//! This module implements the commands from stdin input. It's
//! the main driver for the program.

use std::io;

pub struct Command {
}

impl Command {
    pub fn run(mut buf: &mut impl io::BufRead) -> Result<(), io::Error> {
        // Read one line of setup input to determine the blocks size.
        // Reading in a loop so we can re-prompt the user if they
        // enter an invalid value.
        let mut input = String::new();
        
        loop {
            input.clear();
            buf.read_line(&mut input)?;
            
            let boxes_and_dimensions: Vec<usize> = 
                input.split_whitespace()
                     .map(|s| s.parse().unwrap())
                     .collect();
            
            if boxes_and_dimensions.len() != 2 {
                panic!("The first input line must be two unsigned integers.")
            }
            
            let num_boxes = boxes_and_dimensions[0];
            let num_dimensions = boxes_and_dimensions[1];
            
            println!("num_boxes = {}, num_dimensions = {}", num_boxes, num_dimensions);
            println!("{:?}", boxes_and_dimensions);
            
            for i in 0..num_boxes {
                input.clear();
                buf.read_line(&mut input)?;
                
                let box_vec: Vec<usize> = 
                    input.split_whitespace()
                         .map(|s| s.parse().unwrap())
                         .collect();
                
                if box_vec.len() != num_dimensions {
                    panic!(
                        "Box {} input line must be {} unsigned integers.",
                        i + 1,
                        num_dimensions
                    );
                }
                
                println!("Box {} = {:?}", i + 1, box_vec);
            }
            
            println!("The longest nested string is ...");
        }
        //loop {
        //        for boxes_input in input.trim()
        //                                .split_whitespace()
        //                                .map(|a| a.parse::<usize>())
        //                                .collect() {
        //            println!("num_boxes = {}, num_dimensions = {}", boxes_input[1], boxes_input[2]);
        //        }
        //    }
        //    if let Ok(num_blocks) = setup.trim().parse::<usize>() {
        //        if num_blocks == 0 {
        //            eprintln!("Error! Blocks size must be greater than 0.");
        //            continue;
        //        }
        //        
        //        // Create a Robot instance containing Blocks of the
        //        // specified size, and run it.
        //        return Robot::new(num_blocks).main_loop(&mut buf);
        //    }
        //    else if setup.trim() == "q" || setup.trim() == "quit" {
        //        return Ok(());
        //    }
        //    else {
        //        eprintln!("Error! Please enter the desired blocks size as a positive integer.");
        //    }
        //}
    }
}
