/// Consider the following algorithm:
///  1. input n
///  2. print n
///  3. if n = 1 then STOP
///  4. if n is odd then n ←− 3n + 1
///  5. else n ←− n/2
///  6. GOTO 2
/// Given the input 22, the following sequence of numbers will be printed
/// 22 11 34 17 52 26 13 40 20 10 5 16 8 4 2 1
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
