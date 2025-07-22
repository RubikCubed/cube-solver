mod cube;

use crate::cube::*;

fn main() {
    let scramble = R * U * U * F * L;
    scramble.print_net();

    let start = std::time::Instant::now();

    if let Some(path) = ids(scramble, 6) {
        let elapsed = start.elapsed();
        eprintln!("Elapsed: {:?}", elapsed);
        println!(
            "Solution Found: {}",
            path.into_iter()
                .map(|(s, _)| s)
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
