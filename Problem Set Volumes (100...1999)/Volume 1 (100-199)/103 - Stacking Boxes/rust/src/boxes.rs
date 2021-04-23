//! `rust` crate.
//!
//! Author: Jonathan Sawyer <jonmsawyer[at]gmail.com>
//!
//! Date: 2020-06-19
//!
//! This crate implements one solution to the [Stacking Boxes problem],
//! which calculates the longest string of stacking boxes.
//!
//! [Stacking Boxes problem]: https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=3&page=show_problem&problem=39

use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct Box_ {
     pub box_: Vec<usize>,
}

impl PartialOrd for Box_ {
    fn partial_cmp(&self, other: &Box_) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Box_ {
    fn cmp(&self, other: &Box_) -> Ordering {
        let self_box_len = self.box_.len();
        let other_box_len = other.box_.len();
        
        if self_box_len != other_box_len {
            return self.box_.cmp(&other.box_);
        }
        
        let mut is_less = true;
        let mut is_greater = true;
        
        for i in 0..self_box_len {
            if self.box_[i] >= other.box_[i] {
                is_less = false;
            }
            
            if self.box_[i] <= other.box_[i] {
                is_greater = false;
            }
        }
        
        if is_less {
            return Ordering::Less;
        }
        
        if is_greater {
            return Ordering::Greater;
        }
        
        Ordering::Equal
    }
}

impl PartialEq for Box_ {
    fn eq(&self, other: &Box_) -> bool {
        let self_box_len = self.box_.len();
        let other_box_len = other.box_.len();
        
        if self_box_len != other_box_len {
            return false;
        }
        
        let mut is_less = true;
        let mut is_greater = true;
        
        for i in 0..self_box_len {
            if self.box_[i] >= other.box_[i] {
                is_less = false;
            }
            
            if self.box_[i] <= other.box_[i] {
                is_greater = false;
            }
        }
        
        if is_less {
            return false;
        }
        
        if is_greater {
            return false;
        }
        
        true
    }
}

#[derive(Debug, PartialEq)]
pub struct Boxes {
    num: usize,
    dimensions: usize,
    boxes: Vec<Box_>,
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn box_is_less_than_box() {
        let box1 = Box_ {
            box_: vec![1, 2, 3, 4, 5],
        };
        
        let box2 = Box_ {
            box_: vec![2, 3, 4, 5, 6],
        };
        
        assert!(box1 < box2);
        assert!(box2 > box1);
    }
    
    #[test]
    fn box_is_greater_than_box() {
        let box1 = Box_ {
            box_: vec![16, 39, 10],
        };
        
        let box2 = Box_ {
            box_: vec![1, 3, 9],
        };
        
        assert!(box1 > box2);
        assert!(box2 < box1);
    }
    
    #[test]
    fn box_is_equal_to_box() {
        let box1 = Box_ {
            box_: vec![1, 2, 3],
        };
        
        let box2 = Box_ {
            box_: vec![1, 2, 3],
        };
        
        assert_eq!(box1, box2);
        
        let box1 = Box_ {
            box_: vec![1, 2, 3],
        };
        
        let box2 = Box_ {
            box_: vec![1, 1, 1],
        };
        
        // If a box can't exactly fit inside another box
        // (dimensions need to be strictly less) then it is
        // considered equal in this context.
        assert_eq!(box1, box2);
    }
    
    #[test]
    fn box_diff_dimensions() {
        let box1 = Box_ {
            box_: vec![2, 3],
        };
        
        let box2 = Box_ {
            box_: vec![1, 2, 3],
        };
        
        assert!(box1 != box2);
        assert!(box1 > box2); // Undefined behavior. Should it pass this test?
        
        let box1 = Box_ {
            box_: vec![2, 3],
        };
        
        let box2 = Box_ {
            box_: vec![2, 3, 1],
        };
        
        assert!(box1 != box2);
        assert!(box1 < box2); // Undefined behavior. Should it pass this test?
        
        let box1 = Box_ {
            box_: vec![2, 3],
        };
        
        let box2 = Box_ {
            box_: vec![2, 3, 3],
        };
        
        assert!(box1 != box2);
    }
}
