//! `command` module
//!
//! Author: Jonathan Sawyer <jonmsawyer[at]gmail.com>
//!
//! Date: 2020-06-04

/// The command state of the attempted command.
#[derive(Debug, PartialEq)]
pub enum CommandState {
    /// Initial state.
    Init,
    
    /// Move state. Indicates that the desired operation is a `move`
    /// from `a` to `b`.
    Move,
    
    /// Pile state. Indicates that the desired operation is a `pile`
    /// from `a` to `b`.
    Pile,
    
    /// Onto state. Indicates that the desired operation is a `move`
    /// or `pile` from `a` `onto` `b`.
    Onto,
    
    /// Over state. Indicates that the desired operation is a `move`
    /// or `pile` from `a` `over` `b`.
    Over,
    
    /// Quit state. Indicates that the desired operation is to print
    /// the blocks world and then exit the program.
    Quit,
    
    /// Print state. Indicates that the desired operation is to print
    /// the blocks world.
    Print,
    
    /// Error state. Indicates that there was an error in parsing the
    /// input command.
    Error,
    
    /// Do state. Indicates that command parsing succeeded within the
    /// appropriate bounds. "Do" the operations indicated provided by
    /// the command.
    Do,
}

/// `Command` struct that, when initialized, holds the state and
/// parameters of commands such as `move a over b`, where `a` and `b`
/// are valid block numbers.
#[derive(Debug)]
pub struct Command {
    /// Holds the original input command such as `move 1 over 3`.
    input: String,
    
    /// When an error occurs during parsing, a `String` error message
    /// will be populated here. If this is set to a non-empty string,
    /// `Command.state` should be `CommandState::Error`.
    pub error_msg: String,
    
    /// `state` attribute that holds the `CommandState` enum instance.
    /// See docs for `CommandState` enum.
    pub state: CommandState,
    
    /// One of `CommandState::Init`, `CommandState::Move` or
    /// `CommandState::Pile`.
    pub from: CommandState,
    
    /// One of `CommandState::Init`, `CommandState::Onto` or
    /// `CommandState::Over`.
    pub to: CommandState,
    
    /// The `a` parameter value for commands such as `move a over b`
    /// where `a` is a valid block number.
    pub a: i32,
    
    /// The `b` parameter value for commands such as `move a over b`
    /// where `b` is a valid block number.
    pub b: i32,
}

impl Command {
    /// Parse the `input` command that is obtained from a string
    /// (usually from `io::stdin`).
    ///
    /// Valid commands take the form of `{verb} {block number}
    /// {adjective/preposition} {block number}` where:
    ///
    /// `{verb}` is one of:
    ///   * `move`: move block `a`
    ///   * `pile`: pile block `a`
    ///
    /// `{adjective/preposition}` is one of:
    ///   * `onto`: move or pile `a` onto `b`
    ///   * `over`: move or pile `a` over `b`
    ///
    /// `{block number}` is:
    ///   * an unsigned integer (including 0) [may be valid or
    ///     invalid]
    ///
    /// # Example
    ///
    /// ```
    /// use rust::command::{Command, CommandState};
    ///
    /// let command = Command::parse(String::from("move 1 onto 3"));
    ///
    /// assert_eq!(command.state, CommandState::Do);
    /// assert_eq!(command.from, CommandState::Move);
    /// assert_eq!(command.to, CommandState::Onto);
    /// assert_eq!(command.a, 1);
    /// assert_eq!(command.b, 3);
    /// ```
    pub fn parse(input: String) -> Command {
        // Convert the `input` string to lowercase, trim the whitespace
        // from the begging and end, and convert it back into a `String`.
        let input = input.to_lowercase().trim().to_string();
        
        // Default states.
        let error_msg = String::new();
        let state = CommandState::Do;
        let mut from = CommandState::Init;
        let mut to = CommandState::Init;
        let mut a = -1;
        let mut b = -1;
        
        // If the user inputs `quit`, `q`, `print`, or `p`, return the
        // appropriate command instances with the proper states.
        match input.as_str() {
            "quit" | "q" => return Command {
                input,
                error_msg,
                state: CommandState::Quit,
                from,
                to,
                a,
                b
            },
            
            "print" | "p" => return Command {
                input,
                error_msg,
                state: CommandState::Print,
                from,
                to,
                a,
                b
            },
            
            _ => {},
        }
        
        // We clone this to get around borrowing rules.
        let input_parts = input.clone();
        
        // Split the input (e.g., `move 1 onto 3`) string into its
        // constituent parts. Results in a `Vec<&str>` instance containing
        // the individual parts of the attempted command.
        let parts: Vec<&str> = input_parts.split_whitespace().collect();
        
        // After checking for our 1-parameter input, we now must have an
        // input string that contains exactly 4 parts. Else return an
        // error.
        if parts.len() != 4 {
            return Command {
                input,
                error_msg: format!("Error! Expected 4 input parameters, got {}", parts.len()),
                state: CommandState::Error,
                from,
                to,
                a,
                b
            };
        }
        
        // Check the first part of the command. It must equal `move` or
        // `pile`. Else return an error.
        if parts[0] != "move" && parts[0] != "pile" {
            return Command {
                input,
                error_msg: format!("Error! `{}` is not a valid command.", parts[0]),
                state: CommandState::Error,
                from,
                to,
                a,
                b
            };
        }
        
        // Check the third part of the command. It must equal `over` or
        // `onto`. Else return an error.
        if parts[2] != "over" && parts[2] != "onto" {
            return Command {
                input,
                error_msg: format!("Error! `{}` is not a valid command.", parts[2]),
                state: CommandState::Error,
                from,
                to,
                a,
                b
            };
        }
        
        // Parse the second part of the command into an unsigned integer,
        // else return an error.
        if let Ok(num) = parts[1].parse::<u32>() {
            a = num as i32;
        }
        else {
            return Command {
                input,
                error_msg: format!("Error! `{}` is not a valid positive integer.", parts[1]),
                state: CommandState::Error,
                from,
                to,
                a,
                b,
            };
        }
        
        // Parse the fourth part of the command into an unsigned integer,
        // else return an error.
        if let Ok(num) = parts[3].parse::<u32>() {
            b = num as i32;
        }
        else {
            return Command {
                input,
                error_msg: format!("Error! `{}` is not a valid positive integer.", parts[3]),
                state: CommandState::Error,
                from,
                to,
                a,
                b,
            };
        }
        
        // Set the appropriate state based on the first part of the
        // input command.
        match parts[0] {
            "move" => from = CommandState::Move,
            "pile" => from = CommandState::Pile,
            _ => {},
        }
        
        // Set the appropriate state based on the third part of the
        // input command.
        match parts[2] {
            "over" => to = CommandState::Over,
            "onto" => to = CommandState::Onto,
            _ => {},
        }
        
        Command { input, error_msg, state, from, to, a, b }
    }
}
