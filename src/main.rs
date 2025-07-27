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

    let start = std::time::Instant::now();
    let corners = Corners::load_pruning_table();
    eprintln!("Loaded corner pruning table in {:?}", start.elapsed());

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
