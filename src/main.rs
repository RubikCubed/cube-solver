mod cube;

use crate::cube::{B, Cube, D, F, L, R, SOLVED, SUPERFLIP, U};

fn main() {
    let cube = &R;
    let rotated = cube.apply(&SOLVED);

    rotated.print_net();

    let result: Cube = [R, R].into_iter().product();

    result.print_net();
}
