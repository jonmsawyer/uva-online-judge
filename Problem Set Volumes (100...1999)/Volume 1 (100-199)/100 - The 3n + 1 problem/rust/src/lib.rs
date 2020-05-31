//! `rust` crate
//!
//! Author: Jonathan Sawyer <jonmsawyer[at]gmail.com>
//!
//! Date: 2020-05-30

/// A `Config` that holds the configuration of the input parameters.
///
/// Given an input of `Config::new("1 10")`, it will return
/// `Config { inputs: ["1", "10"], i: 1, j: 10 }`. This `Config` object
/// is passed into the `max_cycles()` function in this library to obtain
/// a 3-tuple of `(i, j, result)`. 
#[derive(Debug, PartialEq)]
pub struct Config<'a> {
    pub inputs: Vec<&'a str>,
    pub i: u32,
    pub j: u32,
}

impl<'a> Config<'a> {
    /// Given an input string of two unsigned integers, return a `Config`
    /// instance of the inputs split into a `Vec<&str>`, an initial `i`
    /// value, and an initial `j` value.
    ///
    /// Given the input `"1 10"` the resulting config should be:
    ///
    /// ```
    /// use rust::Config;
    ///
    /// // Valid config.
    /// let config = match Config::new("1 10\r\n") {
    ///     Ok(config) => config,
    ///     Err(error) => Config { inputs: vec![], i: 0, j: 0 },
    /// };
    /// let config2 = Config {
    ///     inputs: vec!["1", "10"],
    ///     i: 1,
    ///     j: 10,
    /// };
    /// assert_eq!(config, config2);
    /// ```
    ///
    /// An invalid config yields:
    ///
    /// ```
    /// // Invalid config.
    /// let config = match Config::new("asdf asdf\r\n") {
    ///     Ok(config) => config,
    ///     Err(error) => Config { inputs: vec![], i: 0, j: 0 },
    /// };
    /// let config2 = Config {
    ///     inputs: vec![],
    ///     i: 0,
    ///     j: 0,
    /// };
    /// assert_eq!(config, config2);
    /// ```
    pub fn new(input: &'a str) -> Result<Config<'a>, String> {
        // Split the input string into a `Vec<&str>` instance delimited
        // by whitespace (e.g, ' ', '\t', "\r\n", etc).
        let inputs: Vec<&str> = input.split_whitespace().collect();
        
        if inputs.len() > 2 {
            return Err(
                format!("too many input parameters, expected 2, got {}", inputs.len())
            );
        }
        
        if inputs.len() < 2 {
            return Err(
                format!("too few input parameters, expected 2, got {}", inputs.len())
            );
        }
        
        // Parse the first string parameter in `inputs` into an
        // unsigned integer.
        let i = match inputs[0].parse::<u32>() {
            Ok(number) => number,
            Err(e) => return Err(format!("error: {}", e)),
        };
        
        // Parse the second string parameters in `inputs` into an
        // unsigned integer.
        let j = match inputs[1].parse::<u32>() {
            Ok(number) => number,
            Err(e) => return Err(format!("error: {}", e)),
        };
        
        Ok(Config { inputs, i, j })
    }
}

/// Returns a `Vec<u32>` containing the elements of the cycle length
/// of `n` with respect to the "3n + 1 problem".
///
/// Consider the following algorithm:
///
///  1. input n
///  2. print n
///  3. if n = 1 then STOP
///  4. if n is odd then n ← 3n + 1
///  5. else n ← n/2
///  6. GOTO 2
///
/// Given the input 22, the following sequence of numbers will be:
///
/// ```
/// use rust::cycles;
/// assert_eq!(
///     cycles(22),
///     vec![22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1],
/// );
/// ```
pub fn cycles(mut n: u32) -> Vec<u32> {
    // This is where we'll store the cycle length values.
    let mut cycle_vec = Vec::<u32>::new();
    
    // Loop until `n == 1`.
    loop {
        if n == 1 {
            cycle_vec.push(1);
            break;
        }
        else {
            cycle_vec.push(n);
            if n % 2 == 0 { // n is even
                n /= 2;
            }
            else { // n is odd
                n = (3 * n) + 1
            }
        }
    }
    cycle_vec
}

/// Returns a 3-tuple of the initial `Config` parameter `i`, and `j`,
/// and the `result` of the maximum length for all cycle lengths between
/// `i`, and `j`.
pub fn max_cycles(config: Config) -> (u32, u32, u32) {
    // We store the maximum size of each cycle length here.
    let mut result: usize = 0;
    
    // If we have invalid inputs, return (0, 0, 0) 3-tuple.
    if config.inputs.len() == 0 || (config.i == 0 && config.j == 0) {
        return (0, 0, 0);
    }
    
    // Arrange the input parameters `i` and `j` into their
    // `(min, max)` values.
    let (min, max) = match config.i < config.j {
        true => (config.i, config.j),
        false => (config.j, config.i),
    };
    
    // Loop from the `min` value to the `max` value (inclusive)
    // and computing the resulting length of each `cycle_vec` and
    // storing the maximum size into `result`.
    for n in min..=max {
        let cycle_vec = cycles(n);
        result = cycle_vec.len().max(result);
        //println!("cycle_vec = {:?}, cycle length = {}", cycle_vec, cycle_vec.len());
    }
    
    (config.i, config.j, result as u32)
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn correct_cycle_vec_and_length() {
        let cycle_vec = cycles(22);
        assert_eq!(
            vec![22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1],
            cycle_vec
        );
        assert_eq!(16, cycle_vec.len());
        
        let cycle_vec = cycles(10);
        assert_eq!(vec![10, 5, 16, 8, 4, 2, 1], cycle_vec);
        assert_eq!(7, cycle_vec.len());
    }
    
    #[test]
    fn correct_max_cycles_result() {
        let config = Config {
            inputs: vec!["1", "10"],
            i: 1,
            j: 10,
        };
        
        let (i, j, result) = max_cycles(config);
        
        assert_eq!(i, 1);
        assert_eq!(j, 10);
        assert_eq!(result, 20);
        
        let config = Config {
            inputs: vec!["10", "1"],
            i: 10,
            j: 1,
        };
        
        let (i, j, result) = max_cycles(config);
        
        assert_eq!(i, 10);
        assert_eq!(j, 1);
        assert_eq!(result, 20);
    }
}
