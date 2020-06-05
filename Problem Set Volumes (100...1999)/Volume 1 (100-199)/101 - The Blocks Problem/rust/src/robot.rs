//! `robot` module
//!
//! Author: Jonathan Sawyer <jonmsawyer[at]gmail.com>
//!
//! Date: 2020-06-04
//!
//! # Usage
//!
//! ```no_run
//! fn main() -> std::io::Result<()> {
//!     use std::io;
//!     use rust::robot::Robot;
//!     
//!     let stdin = io::stdin();
//!     let mut handle = stdin.lock();
//!     
//!     Robot::run(&mut handle).unwrap();
//!         
//!     Ok(())
//! }
//! ```
//!
//! or
//!
//! ```no_run
//! fn main() -> std::io::Result<()> {
//!     use std::io::BufReader;
//!     use std::fs::File;
//!     use rust::robot::Robot;
//!     
//!     let f = File::open("log.txt")?;
//!     let mut reader = BufReader::new(f);
//!     
//!     Robot::run(&mut reader).unwrap();
//!     
//!     Ok(())
//! }
//! ```

use std::io;

use crate::blocks::{Blocks, BlockState};
use crate::command::{Command, CommandState};

/// A robot struct that both runs and provides the main loop to
/// a fictional robot that manipulates blocks on a table.
#[derive(Debug)]
pub struct Robot {
    pub blocks: Blocks,
}

impl Robot {
    /// Return a new `Robot` instance containing a `Blocks` world.
    pub fn new(num_blocks: u32) -> Robot {
        let blocks = match Blocks::new(num_blocks) {
            Ok(blocks) => blocks,
            Err(_) => Blocks {
                state: BlockState::Init,
                world: vec![vec![0]],
                a: None,
                b: None,
            },
        };
        Robot {
            blocks
        }
    }
    
    /// Loop through the input buffer (`buf`), reading each line of input
    /// until the user `quit`s. `buf` must implement `io::BufRead` (and
    /// thus can be from `io::stdin` or `io::BufReader`).
    ///
    /// # Example
    ///
    /// ```no_run
    /// fn main() -> std::io::Result<()> {
    ///     use std::io;
    ///     use rust::robot::Robot;
    ///     
    ///     let stdin = io::stdin();
    ///     let mut handle = stdin.lock();
    ///     
    ///     Robot::run(&mut handle).unwrap();
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// or
    ///
    /// ```no_run
    /// fn main() -> std::io::Result<()> {
    ///     use std::io::BufReader;
    ///     use std::fs::File;
    ///     use rust::robot::Robot;
    ///     
    ///     let f = File::open("log.txt")?;
    ///     let mut reader = BufReader::new(f);
    ///     
    ///     Robot::run(&mut reader).unwrap();
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn run(mut buf: &mut impl io::BufRead) -> Result<(), io::Error> {
        // Read one line of setup input to determine the blocks size.
        // Reading in a loop so we can re-prompt the user if they
        // enter an invalid value.
        loop {
            let mut setup = String::new();
            buf.read_line(&mut setup)?;
            
            if let Ok(num_blocks) = setup.trim().parse::<u32>() {
                if num_blocks == 0 {
                    eprintln!("Error! Blocks size must be greater than 0.");
                    continue;
                }
                
                // Create a Robot instance containing Blocks of the
                // specified size, and run it.
                return Robot::new(num_blocks).main_loop(&mut buf);
            }
            else if setup.trim() == "q" || setup.trim() == "quit" {
                return Ok(());
            }
            else {
                eprintln!("Error! Please enter the desired blocks size as a positive integer.");
            }
        }
    }
    
    /// The main program loop.
    ///
    /// This loop runs until a `quit` command is received. On each
    /// iteration of the loop the `Robot` waits for a line of input,
    /// which it parses into a `Command` that it then executes.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::io;
    /// use rust::robot::Robot;
    ///
    /// let num_blocks = 10;
    /// let stdin = io::stdin();
    /// let mut buf = stdin.lock();
    ///
    /// Robot::new(num_blocks).main_loop(&mut buf);
    /// ```
    pub fn main_loop(&mut self, buf: &mut impl io::BufRead) -> Result<(), io::Error> {
        let mut input = String::new();
        
        loop {
            // Empty the buffer without touching its capacity.
            input.clear();
            
            // Read a command from our input.
            buf.read_line(&mut input)?;
            
            // Parse the input command.
            let command = Command::parse(&input);
            
            // Based on the state of the parsed command, we match the
            // command state with its appropriate arms to produce the
            // desired output.
            match command.state {
                // This should theoretically never happen.
                CommandState::Init => eprintln!("Command is init??"),
                
                // Print the state of the blocks world onto
                // `std::io::stdout`.
                CommandState::Print => self.blocks.print(),
                
                // Print the state of the blocks world onto
                // `std::io::stdout` and then quit the program.
                CommandState::Quit => { self.blocks.print(); break; },
                
                // During development, we printed the error messages
                // onto `std::io::stdout`, but since this program
                // can't output any error messages, we ignore them.
                CommandState::Error => {},
                
                // Perform the requested command operation. This is
                // where the magic happens.
                CommandState::Do => {
                    match command.from {
                        // Move `a`.
                        CommandState::Move => {
                            match command.to {
                                // Over `b`.
                                CommandState::Over => {
                                    self.blocks.move_a(command.a as u32).over_b(command.b as u32);
                                },
                                
                                // Onto `b`.
                                CommandState::Onto => {
                                    self.blocks.move_a(command.a as u32).onto_b(command.b as u32);
                                },
                                
                                // Catch all.
                                _ => {},
                            }
                        },
                        
                        // Pile `a`.
                        CommandState::Pile => {
                            match command.to {
                                // Over `b`.
                                CommandState::Over => {
                                    self.blocks.pile_a(command.a as u32).over_b(command.b as u32);
                                },
                                
                                // Onto `b`.
                                CommandState::Onto => {
                                    self.blocks.pile_a(command.a as u32).onto_b(command.b as u32);
                                },
                                _ => {},
                            }
                        },
                        
                        // Catch all.
                        _ => {},
                    }
                },
                
                // Catch all.
                _ => {},
            }
        }
        
        Ok(())
    }
}
