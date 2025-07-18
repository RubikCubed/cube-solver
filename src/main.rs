mod cube;

use crate::cube::{R, SOLVED, U};

fn main() {
    let cube = SOLVED;
    //wslFdiJOHlet rotated = cube.apply(&U).apply(&R);

    //cube.print_net();
    //rotated.print_net()
    println!("{:?}", cube.to_facelets());
}
