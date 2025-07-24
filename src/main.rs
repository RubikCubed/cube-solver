#![feature(stmt_expr_attributes)]

mod cube;
mod mv;

use crate::cube::*;

fn main() {
    let scramble = R * U * U * F * L * B;
    //let scramble = SUPERFLIP;
    scramble.print_net();

    dbg!(
        scramble.corner_perm_coordinate(),
        scramble.corner_orientation_coordinate()
    );

    let start = std::time::Instant::now();

    if let Some(path) = ida(scramble, 10, (ZeroBound, EOBound)) {
        let elapsed = start.elapsed();
        eprintln!("Elapsed: {:?}", elapsed);
        println!(
            "Solution Found: {}",
            path.into_iter()
                .map(|m| m.to_str())
                .collect::<Vec<_>>()
                .join(" ")
        );

        dbg!(
            SOLVED.corner_perm_coordinate(),
            SOLVED.corner_orientation_coordinate()
        );
    }
}
