use rust::{Blocks, BlockState};

fn main() {
    let mut blocks = match Blocks::new(8) {
        Ok(blocks) => blocks,
        Err(_) => Blocks {
            state: BlockState::Init,
            world: vec![vec![0]],
            a: None,
            b: None,
        },
    };
    
    blocks.print();
    
    blocks.move_a(1).onto_b(2);
    blocks.move_a(2).onto_b(0);
    blocks.move_a(3).onto_b(1);
    blocks.move_a(0).onto_b(3);
    blocks.move_a(1).onto_b(0);
    
    blocks.move_a(1).over_b(2);
    blocks.move_a(3).over_b(0);
    blocks.move_a(0).over_b(2);
    
    blocks.pile_a(1).onto_b(3);
    blocks.pile_a(0).onto_b(2);
    blocks.pile_a(3).onto_b(2);
    blocks.pile_a(33).onto_b(45);
    
    blocks.pile_a(7).over_b(6);
    blocks.pile_a(6).over_b(5);
    blocks.pile_a(3).over_b(5);
    blocks.pile_a(5).over_b(0);
    
    blocks.move_a(0).over_b(4);
    blocks.pile_a(4).over_b(0);
}
