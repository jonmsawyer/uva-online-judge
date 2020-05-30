//! rust crate
//! Author: Jonathan Sawyer <jonmsawyer[at]gmail.com>

#[derive(Debug)]
pub struct Config<'a> {
    pub inputs: Vec<&'a str>,
    pub i: u32,
    pub j: u32,
}

impl<'a> Config<'a> {
    pub fn new(input: &'a str) -> Result<Config<'a>, String> {
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
        
        let i = match inputs[0].parse::<u32>() {
            Ok(number) => number,
            Err(e) => return Err(format!("error: {:?}", e)),
        };
        
        let j = match inputs[1].parse::<u32>() {
            Ok(number) => number,
            Err(e) => return Err(format!("error: {:?}", e)),
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
/// vec![22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1]
/// ```
pub fn cycles(mut n: u32) -> Vec<u32> {
    let mut cycle_vec = Vec::<u32>::new();
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

pub fn max_cycles(config: Config) -> (u32, u32, u32) {
    let mut max: usize = 0;
    
    for n in config.i..=config.j {
        let cycle_vec = cycles(n);
        max = cycle_vec.len().max(max);
        //println!("cycle_vec = {:?}, cycle length = {}", cycle_vec, cycle_vec.len());
    }
    
    (config.i, config.j, max as u32)
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
}
