mod cube;

use crate::cube::*;

fn main() {
    let scramble = R * U * U * F * F;
    scramble.print_net();

    if let Some(path) = ids(scramble, 6) {
        println!(
            "Solution Found: {}",
            path.into_iter()
                .map(|(s, _)| s)
                .collect::<Vec<_>>()
                .join(" ")
        );
        SOLVED.print_net();
    }
}
