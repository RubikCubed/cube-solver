#![feature(stmt_expr_attributes)]
#![feature(generic_const_exprs)]

mod cube;
mod heuristics;
mod mv;
mod pruning_table;
mod puzzle;

use cube::*;
use pruning_table::*;

use puzzle::Puzzle;

fn main() {
    //let scramble = R * U * U * F * L * B * D * R;
    let scramble = SUPERFLIP;

    scramble.print_net();

    eprintln!("Loading EO pruning table...");
    let start = std::time::Instant::now();

    let eo_pruning_table: Box<PruningTable<Cube, EO>> = load_pruning_table("eo_pruning_table.bin");
    eprintln!("Loaded EO pruning table in {:?}", start.elapsed());

    eprintln!("Loading Corner pruning table...");
    let start = std::time::Instant::now();
    let corner_pruning_table: Box<PruningTable<Cube, (CornerOrientation, CornerPermutation)>> =
        load_pruning_table("corner_pruning_table.bin");
    eprintln!("Loaded corner pruning table in {:?}", start.elapsed());

    let start = std::time::Instant::now();

    if let Some(path) = ida(
        scramble,
        20,
        (corner_pruning_table.as_ref(), eo_pruning_table.as_ref()),
    ) {
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

pub fn load_pruning_table<S, T>(path: impl AsRef<std::path::Path>) -> Box<PruningTable<S, T>>
where
    S: Puzzle + Clone + std::ops::Mul<crate::mv::Move, Output = S>,
    T: Coordinate<S>,
    [u8; T::MAX]: Sized,
{
    match std::fs::read(path.as_ref()) {
        Ok(data) => PruningTable::new(data.try_into().unwrap()),
        Err(e) => {
            eprintln!("Error reading pruning table: {}", e);
            let table = PruningTable::generate();
            std::fs::write(path, table.as_ref()).expect("Failed to write pruning table");
            table
        }
    }
}
