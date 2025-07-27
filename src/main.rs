#![feature(stmt_expr_attributes)]

mod cube;
mod heuristics;
mod mv;

use crate::cube::*;
use crate::heuristics::*;
use crate::mv::Move::*;

fn main() {
    //let scramble = R * U * U * F * L * B * D * R;
    let scramble = SUPERFLIP;
    scramble.print_net();

    let corners = Corners::generate();

    let start = std::time::Instant::now();

    if let Some(path) = ida(scramble, 20, corners.as_ref()) {
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
