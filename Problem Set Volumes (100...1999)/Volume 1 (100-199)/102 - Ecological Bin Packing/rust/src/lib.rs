//! `rust` crate
//!
//! Author: Jonathan Sawyer <jonmsawyer[at]gmail.com>
//!
//! Date: 2020-06-06

use std::io;

/// `Bin` enum. Has three variants, `Bin::One`, `Bin::Two`,
/// and `Bin::Three`. Each variant holds a 3-tuple of unsigned
/// integers. The first element in each bin corresponds to the
/// color Brown. The second element in each bin corresponds to
/// the color Green. The third element in each bin corresponds
/// to the color Clear.
#[derive(Debug, PartialEq)]
enum Bin {
    /// Bin One. Colors are: `(Brown, Green, Clear)`.
    One(usize, usize, usize),
    
    /// Bin Two. Colors are: `(Brown, Green, Clear)`.
    Two(usize, usize, usize),
    
    /// Bin Three. Colors are: `(Brown, Green, Clear)`.
    Three(usize, usize, usize),
}

/// `BinState` enum. Each `Bins` instance has a `state`
/// field which corresponds to one of these three variants:
/// `BinState::Ok`, `BinState::Err`, and `BinState::Quit`.
#[derive(Debug, PartialEq)]
enum BinState {
    /// `BinState::Ok` variant. Indicates that the input
    /// parsing was successful (9 input parameters, all
    /// unsigned integers).
    Ok,
    
    /// `BinState::Err` variant. Indicates that there was an
    /// error during input parameter parsing: 1) there was not
    /// exactly 9 input parameters; or 2) one of the 9 input
    /// parameters did not parse into an unsigned integer.
    Err,
    
    /// `BinState::Quit` variant. Indicates that the user
    /// input no parameters (an empty input string) or that
    /// the user entered "q" or "quit" as input.
    Quit,
}

/// `Bins` struct. Contains the `state` and the bins (one, two,
/// and three) that holds the Brown, Green, and Clear recycling
/// glass.
#[derive(Debug, PartialEq)]
pub struct Bins {
    /// Contains the `BinState` enum variant that indicates the
    /// status of the `Bins` instance.
    state: BinState,
    
    /// Contains `Bin::One`.
    bin1: Bin,
    
    /// Contains `Bin::Two`.
    bin2: Bin,
    
    /// Contains `Bin::Three`.
    bin3: Bin,
}

impl Bins {
    //
    // Public methods.
    //
    
    /// Run the `Bins` simulation. Takes as an argument `buf` that
    /// implements the `io::BufRead` trait. For each line of input,
    /// and based on the internal state, parse out that input into
    /// 9 unsigned integers, each integer representing the number
    /// of Brown, Green, and Clear bottles in each bin. When
    /// parsing is successful, output onto `io::stdout` the optimal
    /// bin order and smallest count of bottle transfers for each
    /// bin. If there is an error in parsing the input, output onto
    /// `io::stderr` an error message. When the user inputs "q",
    /// "quit", or an empty line of input, quit the program.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::io;
    /// 
    /// use rust::Bins;
    /// 
    /// fn main() -> std::io::Result<()> {
    ///     let stdin = io::stdin();
    ///     let mut _reader = stdin.lock();
    ///     
    ///     Bins::run(&mut _reader);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn run(mut buf: &mut impl io::BufRead) {
        loop {
            let bins = Bins::new(&mut buf);
            
            match bins.state {
                BinState::Ok => {
                    let (output, count) = bins.calculate();
                    println!("{} {}", output, count);
                },
                BinState::Err => {
                    eprintln!("Error! Invalid parameters.");
                },
                BinState::Quit => {
                    break;
                }
            }
        }
    }
    
    /// Return a new `Bins` instance of the parsing result. Takes
    /// as an argument `buf` that implements the `io::BufRead`
    /// trait. For the line of input, parse out 9 unsigned
    /// integers into their respective bins, each representing
    /// the number of Brown, Green, and Clear glass bottles,
    /// respectively.
    fn new(mut buf: &mut impl io::BufRead) -> Bins {
        if let Ok((a, b, c, d, e, f, g, h, i)) = Bins::read_line(&mut buf) {
            if a == -1 || b == -1 || c == -1 ||
               d == -1 || e == -1 || f == -1 ||
               g == -1 || h == -1 || i == -1
            {
                Bins::error()
            }
            else if a == -2 || b == -2 || c == -2 ||
                    d == -2 || e == -2 || f == -2 ||
                    g == -2 || h == -2 || i == -2
            {
                Bins::quit()
            }
            else {
                Bins::ok(a as usize, b as usize, c as usize,
                         d as usize, e as usize, f as usize,
                         g as usize, h as usize, i as usize)
            }
        }
        else {
            Bins::error()
        }
    }
    
    //
    // Private methods.
    //
    
    /// Read a line of input from `buf` and return a `Result<T>`
    /// variant that either holds the 9 integers read from input,
    /// or an `io::Error`.
    fn read_line(buf: &mut impl io::BufRead) -> io::Result<(isize, isize, isize,
                                                            isize, isize, isize,
                                                            isize, isize, isize)>
    {
        let mut input = String::new();
        
        buf.read_line(&mut input)?;
        
        Ok(Bins::parse(input))
    }
    
    /// Given an input string, parse out the command from the
    /// user and process the command accordingly. Return values
    /// of `-1`s indicated invalid parameters. Return values of
    /// `-2`s indicates that the user wants to quit the program.
    ///
    /// If any return result contains a `-1` value, it's treated
    /// like an invalid parameters error (either parameter count
    /// was not exactly 9 or could not parse exactly 9 unsigned
    /// integers).
    fn parse(input: String) -> (isize, isize, isize,
                                isize, isize, isize,
                                isize, isize, isize)
    {
        match input.trim().to_lowercase().as_str() {
            "quit" | "q" | "" => return (-2, -2, -2,
                                         -2, -2, -2,
                                         -2, -2, -2),
            _ => {},
        }
        
        let parts: Vec<Result<usize, _>> = input
            .split_whitespace()
            .map(|x| x.parse::<usize>())
            .collect();
        
        if parts.len() != 9 {
            return (-1, -1, -1,
                    -1, -1, -1,
                    -1, -1, -1)
        }
        
        let mut a = -1; // Bin One, Brown
        let mut b = -1; // Bin One, Green
        let mut c = -1; // Bin One, Clear
        let mut d = -1; // Bin Two, Brown
        let mut e = -1; // Bin Two, Green
        let mut f = -1; // Bin Two, Clear
        let mut g = -1; // Bin Three, Brown
        let mut h = -1; // Bin Three, Green
        let mut i = -1; // Bin Three, Clear
        
        if let Ok(a_) = parts[0] {
            a = a_ as isize;
        }
        
        if let Ok(b_) = parts[1] {
            b = b_ as isize;
        }
        
        if let Ok(c_) = parts[2] {
            c = c_ as isize;
        }
        
        if let Ok(d_) = parts[3] {
            d = d_ as isize;
        }
        
        if let Ok(e_) = parts[4] {
            e = e_ as isize;
        }
        
        if let Ok(f_) = parts[5] {
            f = f_ as isize;
        }
        
        if let Ok(g_) = parts[6] {
            g = g_ as isize;
        }
        
        if let Ok(h_) = parts[7] {
            h = h_ as isize;
        }
        
        if let Ok(i_) = parts[8] {
            i = i_ as isize;
        }
        
        (a, b, c,
         d, e, f,
         g, h, i)
    }
    
    /// Return a `Bins` instance indicating an error has
    /// occured during parsing.
    fn error() -> Bins {
        Bins {
            state: BinState::Err,
            bin1: Bin::One(0, 0, 0),
            bin2: Bin::Two(0, 0, 0),
            bin3: Bin::Three(0, 0, 0),
        }
    }
    
    /// Return a `Bins` instance indicating that the user
    /// wants to quit the program.
    fn quit() -> Bins {
        Bins {
            state: BinState::Quit,
            bin1: Bin::One(0, 0, 0),
            bin2: Bin::Two(0, 0, 0),
            bin3: Bin::Three(0, 0, 0),
        }
    }
    
    /// Return a `Bins` instance with each stack of bottles
    /// in their proper place.
    fn ok(a: usize, b: usize, c: usize,
          d: usize, e: usize, f: usize,
          g: usize, h: usize, i: usize) -> Bins
    {
        Bins {
            state: BinState::Ok,
            bin1: Bin::One(a, b, c),
            bin2: Bin::Two(d, e, f),
            bin3: Bin::Three(g, h, i)
        }
    }
    
    /// Calculate the shortest combination of bottle moves
    /// from one bin to another. Return a 2-tuple containing
    /// the output bin arrangement and the smallest count of
    /// bottle moves.
    fn calculate(&self) -> (String, usize) {
        let mut outputs = vec![          // E.g.: vec![
            self.bgc(),                  //           ("BGC",  4),
            self.bcg(),                  //           ("BCG", 10),
            self.gbc(),                  //           ("GBC",  2),
            self.gcb(),                  //           ("GCB",  2),
            self.cbg(),                  //           ("CBG",  8),
            self.cgb(),                  //           ("CGB", 10),
        ];                               //       ];
        
        //      sort               by count           by output
        outputs.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        
        (outputs[0].0.clone(), outputs[0].1) // E.g.: ("GBC", 2)
    }
    
    /// Calculate the `BGC` bin arrangement. Finds the number of
    /// bottles that would need to be moved for this arrangement.
    /// Returns a 2-tuple containing a String and the count of
    /// bottle moves.
    fn bgc(&self) -> (String, usize) {
        let output = String::from("BGC");
        
        //              B
        if let Bin::One(_, b, c) = self.bin1 {
            //                 G
            if let Bin::Two(d, _, f) = self.bin2 {
                //                      C
                if let Bin::Three(g, h, _) = self.bin3 {
                    return (output, d + g + b + h + c + f);
                }
            }
        }
        
        // This should never happen.
        (output, 0)
    }
    
    /// Calculate the `BCG` bin arrangement. Finds the number of
    /// bottles that would need to be moved for this arrangement.
    /// Returns a 2-tuple containing a String and the count of
    /// bottle moves.
    fn bcg(&self) -> (String, usize) {
        let output = String::from("BCG");
        
        //              B
        if let Bin::One(_, b, c) = self.bin1 {
            //                    C
            if let Bin::Two(d, e, _) = self.bin2 {
                //                   G
                if let Bin::Three(g, _, i) = self.bin3 {
                    return (output, d + g + b + e + c + i);
                }
            }
        }
        
        // This should never happen.
        (output, 0)
    }
    
    /// Calculate the `GBC` bin arrangement. Finds the number of
    /// bottles that would need to be moved for this arrangement.
    /// Returns a 2-tuple containing a String and the count of
    /// bottle moves.
    fn gbc(&self) -> (String, usize) {
        let output = String::from("GBC");
        
        //                 G
        if let Bin::One(a, _, c) = self.bin1 {
            //              B
            if let Bin::Two(_, e, f) = self.bin2 {
                //                      C
                if let Bin::Three(g, h, _) = self.bin3 {
                    return (output, a + g + e + h + c + f);
                }
            }
        }
        
        // This should never happen.
        (output, 0)
    }
    
    /// Calculate the `GCB` bin arrangement. Finds the number of
    /// bottles that would need to be moved for this arrangement.
    /// Returns a 2-tuple containing a String and the count of
    /// bottle moves.
    fn gcb(&self) -> (String, usize) {
        let output = String::from("GCB");
        
        //                 G
        if let Bin::One(a, _, c) = self.bin1 {
            //                    C
            if let Bin::Two(d, e, _) = self.bin2 {
                //                B
                if let Bin::Three(_, h, i) = self.bin3 {
                    return (output, a + d + e + h + c + i);
                }
            }
        }
        
        // This should never happen.
        (output, 0)
    }
    
    /// Calculate the `CBG` bin arrangement. Finds the number of
    /// bottles that would need to be moved for this arrangement.
    /// Returns a 2-tuple containing a String and the count of
    /// bottle moves.
    fn cbg(&self) -> (String, usize) {
        let output = String::from("CBG");
        
        //                    C
        if let Bin::One(a, b, _) = self.bin1 {
            //              B
            if let Bin::Two(_, e, f) = self.bin2 {
                //                   G
                if let Bin::Three(g, _, i) = self.bin3 {
                    return (output, a + g + b + e + f + i);
                }
            }
        }
        
        // This should never happen.
        (output, 0)
    }
    
    /// Calculate the `CGB` bin arrangement. Finds the number of
    /// bottles that would need to be moved for this arrangement.
    /// Returns a 2-tuple containing a String and the count of
    /// bottle moves.
    fn cgb(&self) -> (String, usize) {
        let output = String::from("CGB");
        
        //                    C
        if let Bin::One(a, b, _) = self.bin1 {
            //                 G
            if let Bin::Two(d, _, f) = self.bin2 {
                //                B
                if let Bin::Three(_, h, i) = self.bin3 {
                    return (output, a + d + b + h + f + i);
                }
            }
        }
        
        // This should never happen.
        (output, 0)
    }
}
