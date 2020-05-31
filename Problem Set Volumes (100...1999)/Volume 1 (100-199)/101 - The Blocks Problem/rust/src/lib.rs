//! `rust` crate.
//!
//! Author: Jonathan Sawyer <jonmsawyer[at]gmail.com>
//!
//! Date: 2020-05-30

//use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct Blocks(Vec<(u32, Vec<u32>)>);

impl Blocks {
    pub fn new(elements: u32) -> Result<Blocks, String> {
        if elements < 1 {
            return Err("cannot initialize Blocks with less than 1 element".to_string());
        }
        
        let mut vec_elements = Vec::<(u32, Vec<u32>)>::new();
        for i in 0..elements {
            vec_elements.push((i as u32, vec![i as u32]));
        }
        
        Ok(Blocks(vec_elements))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn blocks_init_new() {
        let blocks = match Blocks::new(3) {
            Ok(blocks) => blocks,
            Err(_error) => Blocks(vec![(0, vec![0])]),
        };
        assert_eq!(
            Blocks(vec![(0, vec![0]), (1, vec![1]), (2, vec![2])]),
            blocks
        );
    }
}
