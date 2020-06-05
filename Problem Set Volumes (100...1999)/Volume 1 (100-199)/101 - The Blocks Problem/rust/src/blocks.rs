//! `blocks` module
//!
//! Author: Jonathan Sawyer <jonmsawyer[at]gmail.com>
//!
//! Date: 2020-06-04

/// The state of the `Blocks` struct during its processing.
/// The state changes depending on the initial command of
/// `move_a()` or `pile_a()`. If there is an invalid order
/// of commands, the blocks state gets set back to
/// `BlockState::Init`.
#[derive(Debug, PartialEq)]
pub enum BlockState {
    /// Initial block state.
    Init,
    
    /// Indicates that the first operation will me a `move`.
    Move,
    
    /// Indicates that the first operation will be a `pile`.
    Pile,
}

/// The `Blocks` instance containing the block state (`Move` or
/// `Pile`), the main blocks structure (vec of vecs), and the `a`
/// and `b` block targets for the operation.
#[derive(Debug, PartialEq)]
pub struct Blocks {
    pub state: BlockState,
    pub world: Vec<Vec<u32>>,
    pub a: Option<u32>,
    pub b: Option<u32>,
}

impl Blocks {
    //
    // Public methods.
    //
    
    /// Create a new `Blocks` instance. `elements` must
    /// be greater than `0` for correct initialization. If
    /// `elements` == `0`, then an `Err()` is returned, else
    /// an `Ok()` variant is returned with the initialized
    /// `Blocks` instance.
    pub fn new(elements: u32) -> Result<Blocks, String> {
        if elements < 1 {
            return Err(String::from("cannot initialize Blocks with less than 1 element"));
        }
        
        // Generate a vec of vecs containing the initial values
        // of their indexes.
        // E.g., `vec![vec![0], vec![1], ..., vec![n]]`
        let vec_elements: Vec<Vec<u32>> = (0..elements).map(|x| vec![x]).collect();
        
        Ok(Blocks {
            world: vec_elements,
            state: BlockState::Init,
            a: None,
            b: None,
        })
    }
    
    /// Given an intial state of `BlockState::Init`, sets the
    /// internal state to `Move` and sets the `a` parameter
    /// appropriately. If the internal state is anything other
    /// than `Init`, then the state of the `Blocks` instance
    /// is reset to initial values.
    pub fn move_a(&mut self, a: u32) -> &mut Self {
        match self.state {
            BlockState::Init => {
                self.state = BlockState::Move;
                self.a = Some(a);
                self.b = None;
            },
            BlockState::Move | BlockState::Pile => {
                self.reset_state();
            },
        }
        
        self
    }
    
    /// Given an intial state of `Init`, sets the internal state
    /// to `Pile` and sets the `a` parameter appropriately. If the
    /// internal state is anything other than `Init`, then the
    /// state of the `Blocks {}` instance is reset to initial values.
    pub fn pile_a(&mut self, a: u32) -> &mut Self {
        match self.state {
            BlockState::Init => {
                self.state = BlockState::Pile;
                self.a = Some(a);
                self.b = None;
            },
            BlockState::Move | BlockState::Pile => {
                self.reset_state();
            },
        }
        
        self
    }
    
    /// Given a state other than `Init`, sets the internal state
    /// to `Init` after moving or piling `a` onto `b`. If the `Init`
    /// state is set, this resets the internal state to default
    /// initial state values.
    pub fn onto_b(&mut self, b: u32) -> &mut Self {
        match self.state {
            BlockState::Init => {
                self.reset_state();
            },
            BlockState::Move => {
                // "move a onto b"
                self.b = Some(b);
                
                // Perform operation.
                self.move_a_onto_b();
                
                // Reset to Init state.
                self.reset_state();
            },
            BlockState::Pile => {
                // "pile a onto b"
                self.b = Some(b);
                
                // Perform operation.
                self.pile_a_onto_b();
                
                // Reset to Init state.
                self.reset_state();
            }
        }
        
        self
    }
    
    /// Given a state other than `Init`, sets the internal state
    /// to `Init` after moving or piling `a` over `b`. If the `Init`
    /// state is set, this resets the internal state to default
    /// initial state values.
    pub fn over_b(&mut self, b: u32) -> &mut Self {
        match self.state {
            BlockState::Init => {
                self.reset_state();
            },
            BlockState::Move => {
                // "move a over b"
                self.b = Some(b);
                
                // Perform operation.
                self.move_a_over_b();
                
                // Reset to Init state.
                self.reset_state();
            },
            BlockState::Pile => {
                // "pile a over b"
                self.b = Some(b);
                
                // Perform operation.
                self.pile_a_over_b();
                
                // Reset to Init state.
                self.reset_state();
            }
        }
        
        self
    }
    
    /// Print onto `std::io::stdout` the internal state of the
    /// `world` attribute for a given `Blocks` instance.
    ///
    /// Should print to something akin to:
    ///
    /// ```ignore
    /// 0: 0 1
    /// 1:
    /// 2: 2
    /// 3: 3 4 5
    /// 4:
    /// 5:
    /// ```
    pub fn print(&self) {
        for (index, vec) in (&self.world).iter().enumerate() {
            print!("{}:", index);
            for &item in vec.iter() {
                print!(" {}", item);
            }
            print!("\n");
        }
    }
    
    //
    // Private methods.
    //
    
    /// `move a onto b`
    ///
    /// Where `a` and `b` are block numbers; puts block
    /// `a` onto block `b` after returning any blocks
    /// that are stacked on top of blocks `a` and `b`
    /// to their initial positions.
    fn move_a_onto_b(&mut self) -> &mut Self {
        let (a, b) = self.get_a_and_b();
        
        //println!("\nmove {} onto {}", a, b);
        
        if self.parameters_invalid() {
            return self.reset_state();
        }
        
        // Get the coordinates of blocks `a` and `b`. This
        // corresponds to the `self.world` vec of vecs.
        let (i, j) = self.where_is(a);
        let (k, l) = self.where_is(b);
        
        // Split the block `a` vector into two slices.
        let (_, right_a) = self.world[i as usize].split_at(j as usize);
        
        let mut block_a: u32;
        
        // Pop off every block on top of block `a` and put those
        // blocks in their original positions.
        for _ in 0..right_a.len() - 1 {
            block_a = self.world[i as usize].pop().unwrap();
            self.world[block_a as usize].push(block_a);
        }
        
        // Finally, pop off block `a` for later use.
        block_a = self.world[i as usize].pop().unwrap();
        
        // Split the block `b` vector into two slices.
        let (_, right_b) = self.world[k as usize].split_at(l as usize);
        
        // Pop off every block on top of block `b` and put those
        // blocks in their original positions.
        for _ in 0..right_b.len() - 1 {
            let block_b = self.world[k as usize].pop().unwrap();
            self.world[block_b as usize].push(block_b);
        }
        
        // Don't pop off block `b` because now we're going to move
        // `a` onto `b`.
        self.world[k as usize].push(block_a);
        
        //self.print();
        
        self
    }
    
    /// `move a over b`
    ///
    /// Where `a` and `b` are block numbers; puts block
    /// `a` onto the top of the stack containing block `b`,
    /// after returning any blocks that are stacked on top
    /// of block `a` to their initial positions.
    fn move_a_over_b(&mut self) -> &mut Self {
        let (a, b) = self.get_a_and_b();
        
        //println!("\nmove {} over {}", a, b);
        
        if self.parameters_invalid() {
            return self.reset_state();
        }
        
        //println!("after parameters invalid()");
        
        // Get the coordinates of blocks `a` and `b`. This corresponds
        // to the `self.world` vec of vecs.
        let (i, j) = self.where_is(a);
        let (k, _) = self.where_is(b);
        
        // Split the block `a` vector into two slices.
        let (_, right_a) = self.world[i as usize].split_at(j as usize);
        
        let mut block_a: u32;
        
        // Pop off every block on top of block `a` and put those
        // blocks in their original positions.
        for _ in 0..right_a.len() - 1 {
            block_a = self.world[i as usize].pop().unwrap();
            self.world[block_a as usize].push(block_a);
        }
        
        // Finally, pop off block `a`.
        block_a = self.world[i as usize].pop().unwrap();
        
        // Put block `a` on top of the stack containing block `b`.
        self.world[k as usize].push(block_a);
        
        //self.print();
        
        self
    }
    
    /// `pile a onto b`
    ///
    /// Where `a` and `b` are block numbers; moves the
    /// pile of blocks consisting of block `a`, and any
    /// blocks that are stacked above block `a`, onto
    /// block `b`. All blocks on top of block `b` are
    /// moved to their initial positions prior to the
    /// pile taking place. The blocks stacked above
    /// block `a` retain their order when moved.
    fn pile_a_onto_b(&mut self) -> &mut Self {
        let (a, b) = self.get_a_and_b();
        
        //println!("\npile {} onto {}", a, b);
        
        if self.parameters_invalid() {
            return self.reset_state();
        }
        
        // Get the coordinates of blocks `a` and `b`. This corresponds
        // to the `self.world` vec of vecs.
        let (i, j) = self.where_is(a);
        let (k, l) = self.where_is(b);
        
        // Split the block `b` vector into two slices.
        let (_, right_b) = self.world[k as usize].split_at(l as usize);
        
        // Pop off every block on top of block `b` and put those
        // blocks in their original positions.
        for _ in 0..right_b.len() - 1 {
            let block_b = self.world[k as usize].pop().unwrap();
            self.world[block_b as usize].push(block_b);
        }
        
        // Split the block `a` vector into two slices.
        let (_, right_a) = self.world[i as usize].split_at(j as usize);
        
        let mut right_a_vec = Vec::<u32>::new();
        
        // Copy the `right_a` slice.
        for idx in 0..right_a.len() {
            right_a_vec.push(right_a[idx]);
        }
        
        // Pop off every block on top of block `a`, including `a`.
        for _ in 0..right_a.len() {
            let _item = self.world[i as usize].pop().unwrap();
        }
        
        // Push the blocks on top of block `a`, including `a`, onto
        // block `b`.
        self.world[k as usize].extend(right_a_vec);
        
        //self.print();
        
        self
    }
    
    /// `pile a over b`
    ///
    /// Where `a` and `b` are block numbers, puts the
    /// pile of blocks consisting of block `a`, and any
    /// blocks that are stacked above block `a`, onto the
    /// top of the stack containing block `b`. The blocks
    /// stacked above block `a` retain their original
    /// order when moved.
    fn pile_a_over_b(&mut self) -> &mut Self {
        let (a, b) = self.get_a_and_b();
        
        //println!("\npile {} over {}", a, b);
        
        if self.parameters_invalid() {
            return self.reset_state();
        }
        
        // Get the coordinates of blocks `a` and `b`. This corresponds
        // to the `self.world` vec of vecs.
        let (i, j) = self.where_is(a);
        let (k, _) = self.where_is(b);
        
        // Split the block `a` vector into two slices.
        let (_, right_a) = self.world[i as usize].split_at(j as usize);
        
        let mut right_a_vec = Vec::<u32>::new();
        
        // Copy the `right_a` slice.
        for idx in 0..right_a.len() {
            right_a_vec.push(right_a[idx]);
        }
        
        // Pop off every block on top of block `a`, including `a`.
        for _ in 0..right_a.len() {
            self.world[i as usize].pop().unwrap();
        }
        
        // Push the blocks on top of block `a`, including `a`, onto
        // block `b`.
        self.world[k as usize].extend(right_a_vec);
        
        //self.print();
        
        self
    }
    
    /// Resets the state of the Blocks struct to the
    /// BlockState::Init state and `a` and `b` = None.
    ///
    /// This can happen when:
    ///
    ///   * `a` and `b` are equal
    ///   * `a` and `b` are in the same stack
    ///   * `a` or `b` are < 0 (-1 for None)
    ///   * `over_b()` or `onto_b()` are called without
    ///     the appropriate state
    ///   * `move_a()` or `pile_a()` are called without
    ///     the appropriate state
    fn reset_state(&mut self) -> &mut Self {
        self.state = BlockState::Init;
        self.a = None;
        self.b = None;
        
        self
    }
    
    /// Return a 2-tuple of the values in the attributes
    /// `a` and `b`.
    fn get_a_and_b(&self) -> (i32, i32) {
        let a = match self.a {
            Some(a) => a as i32,
            None => -1 as i32,
        };
        
        let b = match self.b {
            Some(b) => b as i32,
            None => -1 as i32,
        };
        
        (a, b)
    }
    
    /// Return `true` if the parameters for the operations
    /// to be performed are incorrect. Here are the rules:
    ///
    /// 1. `a` must not equal `b`
    /// 2. `a` and `b` must not be negative
    /// 3. `a` and `b` must not be greater than the length of
    ///    the `world` attribute (a vector)
    /// 4. blocks `a` and `b` must not be in the same stack of
    ///    blocks
    fn parameters_invalid(&self) -> bool {
        let (a, b) = self.get_a_and_b();
        
        if a == b ||
           a < 0 ||
           b < 0 ||
           a as usize >= self.world.len() ||
           b as usize >= self.world.len()
        {
            true
        }
        else {
            self.same_stack()
        }
    }
    
    /// Return `true` if blocks `a` and `b` are in the same stack
    /// of blocks.
    fn same_stack(&self) -> bool {
        let (a, b) = self.get_a_and_b();
        
        for i in (&self.world).iter() {
            if let Some(_) = i.iter().find(|&&x| x == a as u32) {
                if let Some(_) = i.iter().find(|&&x| x == b as u32) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Given a `block` number, return a 2-tuple of the coordinates
    /// for the block.
    ///
    /// Given `blocks.world` vector, find the coordinate for
    ///
    /// ```ignore
    /// vec![
    ///     vec![0, 1, 2],
    ///     vec![],
    ///     vec![]
    /// ]
    /// ```
    ///
    /// such that `blocks.where_is(1)` will return `(0, 1)`.
    fn where_is(&self, block: i32) -> (i32, i32) {
        if block < 0 {
            return (-1, -1);
        }
        
        for (i, stack) in (&self.world).iter().enumerate() {
            for (j, item) in stack.iter().enumerate() {
                if *item == block as u32 {
                    return (i as i32, j as i32);
                }
            }
        }
        
        return (-1, -1);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn blocks_init_new() {
        let blocks = match Blocks::new(3) {
            Ok(blocks) => blocks,
            Err(_error) => Blocks {
                state: BlockState::Init,
                world: vec![vec![0]],
                a: None,
                b: None,
            },
        };
        assert_eq!(
            Blocks {
                state: BlockState::Init,
                world: vec![vec![0], vec![1], vec![2]],
                a: None,
                b: None,
            },
            blocks
        );
    }
    
    #[test]
    fn blocks_init_error() {
        let blocks = match Blocks::new(0) {
            Ok(blocks) => blocks,
            Err(_error) => Blocks {
                state: BlockState::Init,
                world: vec![vec![0]],
                a: None,
                b: None,
            },
        };
        assert_eq!(
            Blocks {
                state: BlockState::Init,
                world: vec![vec![0]],
                a: None,
                b: None,
            },
            blocks
        )
    }
    
    #[test]
    fn blocks_call_multiple_move_a() {
        let mut blocks = match Blocks::new(10) {
            Ok(blocks) => blocks,
            Err(_) => Blocks {
                state: BlockState::Init,
                world: vec![vec![0]],
                a: None,
                b: None,
            }
        };
        
        // Call first `move_a()` method, new state should be
        // `BlockState::Move`.
        blocks.move_a(3);
        assert_eq!(blocks.a, Some(3));
        assert_eq!(blocks.b, None);
        assert_eq!(blocks.state, BlockState::Move);
        
        // Call second `move_a()` method (when state is already
        // `BlockState::Move`).
        blocks.move_a(5);
        assert_eq!(blocks.a, None);
        assert_eq!(blocks.b, None);
        assert_eq!(blocks.state, BlockState::Init);
        
        // Call third `move_a()` method (when state is
        // `BlockState::Init`).
        blocks.move_a(7);
        assert_eq!(blocks.a, Some(7));
        assert_eq!(blocks.b, None);
        assert_eq!(blocks.state, BlockState::Move);
    }
    
    #[test]
    fn blocks_call_multiple_pile_a() {
        let mut blocks = match Blocks::new(10) {
            Ok(blocks) => blocks,
            Err(_) => Blocks {
                state: BlockState::Init,
                world: vec![vec![0]],
                a: None,
                b: None,
            }
        };
        
        // Call first `pile_a()` method, new state should
        // be `BlockState::Pile`.
        blocks.pile_a(3);
        assert_eq!(blocks.a, Some(3));
        assert_eq!(blocks.b, None);
        assert_eq!(blocks.state, BlockState::Pile);
        
        // Call second `pile_a()` method (when state is
        // already `BlockState::Pile`).
        blocks.pile_a(5);
        assert_eq!(blocks.a, None);
        assert_eq!(blocks.b, None);
        assert_eq!(blocks.state, BlockState::Init);
        
        // Call third `move_a()` method (when state is
        // `BlockState::Init`).
        blocks.pile_a(7);
        assert_eq!(blocks.a, Some(7));
        assert_eq!(blocks.b, None);
        assert_eq!(blocks.state, BlockState::Pile);
    }
    
    #[test]
    fn blocks_chain_move_a() {
        let mut blocks = match Blocks::new(10) {
            Ok(blocks) => blocks,
            Err(_) => Blocks {
                state: BlockState::Init,
                world: vec![vec![0]],
                a: None,
                b: None,
            }
        };
        
        // Chain 2 `move_a()` methods. Final state should be
        // `BlockState::Init`.
        blocks.move_a(3).move_a(5);
        assert_eq!(blocks.a, None);
        assert_eq!(blocks.b, None);
        assert_eq!(blocks.state, BlockState::Init);
        
        // Chain 3 `move_a()` methods. Final state should be
        // `BlockState::Move`.
        blocks.move_a(3).move_a(5).move_a(7);
        assert_eq!(blocks.a, Some(7));
        assert_eq!(blocks.b, None);
        assert_eq!(blocks.state, BlockState::Move);
        
        // Call third set of `move_a()` methods. Final state
        // should be `BlockState::Init`.
        blocks.move_a(5);
        assert_eq!(blocks.a, None);
        assert_eq!(blocks.b, None);
        assert_eq!(blocks.state, BlockState::Init);
    }
    
    #[test]
    fn blocks_chain_pile_a() {
        let mut blocks = match Blocks::new(10) {
            Ok(blocks) => blocks,
            Err(_) => Blocks {
                state: BlockState::Init,
                world: vec![vec![0]],
                a: None,
                b: None,
            }
        };
        
        // Chain 2 `pile_a()` methods. Final state should be
        // `BlockState::Init`.
        blocks.pile_a(3).pile_a(5);
        assert_eq!(blocks.a, None);
        assert_eq!(blocks.b, None);
        assert_eq!(blocks.state, BlockState::Init);
        
        // Chain 3 `pile_a()` methods. Final state should be
        // `BlockState::Pile`.
        blocks.pile_a(3).pile_a(5).pile_a(7);
        assert_eq!(blocks.a, Some(7));
        assert_eq!(blocks.b, None);
        assert_eq!(blocks.state, BlockState::Pile);
        
        // Call third set of `pile_a()` methods. Final state
        // should be `BlockState::Init`.
        blocks.pile_a(5);
        assert_eq!(blocks.a, None);
        assert_eq!(blocks.b, None);
        assert_eq!(blocks.state, BlockState::Init);
    }
    
    #[test]
    fn blocks_mix_and_match_move_a_and_pile_a() {
        let mut blocks = match Blocks::new(10) {
            Ok(blocks) => blocks,
            Err(_) => Blocks {
                state: BlockState::Init,
                world: vec![vec![0]],
                a: None,
                b: None,
            }
        };
        
        // Chain `move_a()` and `pile_a()` methods.
        // Final state should be `BlockState::Init`.
        blocks.move_a(3).pile_a(5);
        assert_eq!(blocks.a, None);
        assert_eq!(blocks.state, BlockState::Init);
        
        // Chain `pile_a()`, `move_a(`), and `pile_a()` methods.
        // Final state should be `BlockState::Pile`.
        blocks.pile_a(3).move_a(5).pile_a(7);
        assert_eq!(blocks.a, Some(7));
        assert_eq!(blocks.state, BlockState::Pile);
        
        // Final state should be `BlockState::Init`.
        blocks.move_a(2);
        
        // Chain `move_a()`, `pile_a()`, and `move_a()` methods.
        // Final state should be `BlockState::Move`.
        blocks.move_a(3).pile_a(5).move_a(7);
        assert_eq!(blocks.a, Some(7));
        assert_eq!(blocks.state, BlockState::Move);
        
        // Call third set of `pile_a()` methods.
        // Final state should be `BlockState::Init`.
        blocks.pile_a(5);
        assert_eq!(blocks.a, None);
        assert_eq!(blocks.state, BlockState::Init);
    }
}
