use rust::cycles;

fn main() {
    let n = 22;
    let cycle_vec = cycles(n);
    println!("cycle_vec = {:?}, cycle length = {}", cycle_vec, cycle_vec.len());
}
