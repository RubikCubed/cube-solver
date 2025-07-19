mod cube;

use crate::cube::{B, D, F, L, R, SOLVED, U};

fn main() {
    let cube = SOLVED;
    let rotated = cube.apply(&R).apply(&R).apply(&R);

    cube.print_net();
    rotated.print_net();
    println!("{:?}", rotated);
    println!("{:?}", rotated.to_facelets());
}
