#![feature(stmt_expr_attributes)]

mod cube;
mod heuristics;
mod mv;
mod puzzle;

use crate::cube::*;
use crate::heuristics::*;
use crate::mv::Move::*;

fn main() {
    //let scramble = R * U * U * F * L * B * D * R;
    let scramble = SUPERFLIP;
    scramble.print_net();

    let start = std::time::Instant::now();
    let corners = load_pruning_table();
    let edges = EdgeOrientation::generate();
    eprintln!("Loaded corner pruning table in {:?}", start.elapsed());

    let start = std::time::Instant::now();

    if let Some(path) = ida(scramble, 20, (edges.as_ref(), corners.as_ref())) {
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

pub fn load_pruning_table() -> Box<Corners> {
    match std::fs::read("pruning_table.bin") {
        Ok(data) => Corners::new(data.try_into().unwrap()),
        Err(e) => {
            eprintln!("Error reading pruning table: {}", e);
            Corners::generate()
        }
    }
}
