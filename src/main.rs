#![feature(stmt_expr_attributes)]

mod cube;
mod mv;

use crate::cube::*;

fn main() {
    let scramble = R * U * U * F * L * B;
    let scramble = SUPERFLIP;
    scramble.print_net();

    let start = std::time::Instant::now();

    if let Some(path) = ids(scramble, 10) {
        let elapsed = start.elapsed();
        eprintln!("Elapsed: {:?}", elapsed);
        println!(
            "Solution Found: {}",
            path.into_iter()
                .map(|m| m.to_str())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
