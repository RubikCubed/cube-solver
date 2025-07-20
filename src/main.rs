mod cube;

use crate::cube::{B, Cube, D, F, L, R, SOLVED, SUPERFLIP, U2, dfs, ids};

fn main() {
    if let Some(path) = ids(R, 9) {
        println!("Solution Found: {}", path.join(" "));
    }
}
