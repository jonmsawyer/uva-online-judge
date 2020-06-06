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
#[derive(Debug, PartialEq)]
pub struct Command {
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
    /// let input = String::from("move 1 onto 3");
    /// let command = Command::parse(&input);
    ///
    /// assert_eq!(command.state, CommandState::Do);
    /// assert_eq!(command.from, CommandState::Move);
    /// assert_eq!(command.to, CommandState::Onto);
    /// assert_eq!(command.a, 1);
    /// assert_eq!(command.b, 3);
    /// ```
    pub fn parse(input: &String) -> Command {
        let input = input.trim().to_lowercase();
        
        // Default states.
        let error_msg = String::new();
        let state = CommandState::Do;
        let mut from = CommandState::Init;
        let mut to = CommandState::Init;
        let mut a = -1;
        let mut b = -1;
        
        // If the user inputs `quit`, `q`, `print`, or `p`, return the
        // appropriate command instance with the proper states.
        match input.as_str() {
            "quit" | "q" => return Command {
                error_msg,
                state: CommandState::Quit,
                from,
                to,
                a,
                b
            },
            "print" | "p" => return Command {
                error_msg,
                state: CommandState::Print,
                from,
                to,
                a,
                b
            },
            _ => {},
        }
        
        // Split the input (e.g., `move 1 onto 3`) string into its
        // constituent parts. Results in a `Vec<&str>` instance containing
        // the individual parts of the attempted command.
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        // After checking for our 1-parameter input, we now must have an
        // input string that contains exactly 4 parts. Else return an
        // error.
        if parts.len() != 4 {
            return Command {
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
        
        Command { error_msg, state, from, to, a, b }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn command_parse_move_1_onto_3() {
        let input = String::from("move 1 onto 3\r\n");
        let command = Command::parse(&input);
        
        assert_eq!(command.state, CommandState::Do);
        assert_eq!(command.from, CommandState::Move);
        assert_eq!(command.to, CommandState::Onto);
        assert_eq!(command.a, 1);
        assert_eq!(command.b, 3);
    }
    
    #[test]
    fn command_parse_move_3_over_10() {
        let input = String::from("move 3 over 10\r\n");
        let command = Command::parse(&input);
        
        assert_eq!(command.state, CommandState::Do);
        assert_eq!(command.from, CommandState::Move);
        assert_eq!(command.to, CommandState::Over);
        assert_eq!(command.a, 3);
        assert_eq!(command.b, 10);
    }
    
    #[test]
    fn command_parse_pile_2_onto_100() {
        let input = String::from("pile 2 onto 100\r\n");
        let command = Command::parse(&input);
        
        assert_eq!(command.state, CommandState::Do);
        assert_eq!(command.from, CommandState::Pile);
        assert_eq!(command.to, CommandState::Onto);
        assert_eq!(command.a, 2);
        assert_eq!(command.b, 100);
    }
    
    #[test]
    fn command_parse_pile_200_over_0() {
        let input = String::from("pile 200 over 0\r\n");
        let command = Command::parse(&input);
        
        assert_eq!(command.state, CommandState::Do);
        assert_eq!(command.from, CommandState::Pile);
        assert_eq!(command.to, CommandState::Over);
        assert_eq!(command.a, 200);
        assert_eq!(command.b, 0);
    }
    
    #[test]
    fn command_parse_invalid_number_of_parameters() {
        let input = String::from("move 1 onto 3 right now\r\n");
        let command = Command::parse(&input);
        
        assert_eq!(
            command,
            Command {
                error_msg: format!("Error! Expected 4 input parameters, got 6"),
                state: CommandState::Error,
                from: CommandState::Init,
                to: CommandState::Init,
                a: -1,
                b: -1,
            }
        );
    }
    
    #[test]
    fn command_first_part_invalid_command() {
        let input = String::from("asdf 1 qwer 3\r\n");
        let command = Command::parse(&input);
        
        assert_eq!(
            command,
            Command {
                error_msg: format!("Error! `asdf` is not a valid command."),
                state: CommandState::Error,
                from: CommandState::Init,
                to: CommandState::Init,
                a: -1,
                b: -1,
            }
        );
    }
    
    #[test]
    fn command_third_part_invalid_command() {
        let input = String::from("move 1 qwer 3\r\n");
        let command = Command::parse(&input);
        
        assert_eq!(
            command,
            Command {
                error_msg: format!("Error! `qwer` is not a valid command."),
                state: CommandState::Error,
                from: CommandState::Init,
                to: CommandState::Init,
                a: -1,
                b: -1,
            }
        );
    }
    
    #[test]
    fn command_second_part_invalid_positive_integer() {
        let input = String::from("move -1 onto 3\r\n");
        let command = Command::parse(&input);
        
        assert_eq!(
            command,
            Command {
                error_msg: format!("Error! `-1` is not a valid positive integer."),
                state: CommandState::Error,
                from: CommandState::Init,
                to: CommandState::Init,
                a: -1,
                b: -1,
            }
        );
    }
    
    #[test]
    fn command_fourth_part_invalid_positive_integer() {
        let input = String::from("move 2 onto -3\r\n");
        let command = Command::parse(&input);
        
        assert_eq!(
            command,
            Command {
                error_msg: format!("Error! `-3` is not a valid positive integer."),
                state: CommandState::Error,
                from: CommandState::Init,
                to: CommandState::Init,
                a: 2,
                b: -1,
            }
        );
    }
}
