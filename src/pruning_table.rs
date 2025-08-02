use std::iter::zip;

use crate::cube::Cube;
use crate::heuristics::Heuristic;
use crate::puzzle::Puzzle;

// S = puzzle, T = coordinate
impl<S, T> Heuristic<S> for &PruningTable<S, T>
where
    [u8; T::MAX]: Sized,
    T: Coordinate<S>,
{
    fn lower_bound(self, state: &S) -> u8 {
        let index = T::to_coord(state);
        self.0[index]
    }
}

impl<S, T: Coordinate<S>> AsRef<[u8]> for PruningTable<S, T>
where
    [u8; T::MAX]: Sized,
{
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[repr(transparent)]
pub struct PruningTable<S, T: Coordinate<S>>([u8; T::MAX])
where
    [u8; T::MAX]: Sized;

impl<S: Puzzle + Clone + std::ops::Mul<crate::mv::Move, Output = S>, T: Coordinate<S>>
    PruningTable<S, T>
where
    [u8; T::MAX]: Sized,
{
    pub fn generate() -> Box<Self> {
        eprintln!(
            "Generating pruning table for {}",
            std::any::type_name::<T>()
        );

        let mut table: Box<[u8; T::MAX]> = vec![0; T::MAX].try_into().unwrap();

        let start = std::time::Instant::now();

        let mut total_filled = 1;

        // all the face turns can be set to 1
        for &mv in crate::mv::Move::ALL {
            let new_state = S::solved() * mv;
            let index = T::to_coord(&new_state);
            table[index] = 1;
            total_filled += 1;
        }

        'depth: for depth in 2.. {
            eprintln!("Generating depth {depth}, {total_filled} filled");
            for index in 1..T::MAX {
                if total_filled >= T::MAX {
                    eprintln!("Filled all entries in the table, stopping at depth {depth}");
                    break 'depth;
                }
                let moves_to_solve = table[index];

                if moves_to_solve != depth - 1 {
                    continue;
                }

                let puzzle = T::from_coord(index);

                // apply all moves to the current state, update the new indexes if they aren't set
                for &mv in crate::mv::Move::ALL {
                    let new_state = puzzle.clone() * mv;
                    let new_index = T::to_coord(&new_state);
                    if new_index == 0 {
                        continue;
                    }

                    if table[new_index] == 0 {
                        table[new_index] = depth;
                        total_filled += 1;
                    }
                }
            }
        }

        let elapsed = start.elapsed();
        eprintln!("time taken to generate lookup table: {:?}", elapsed);

        Self::new(table)
    }

    pub fn new(table: Box<[u8; T::MAX]>) -> Box<Self> {
        let p = Box::into_raw(table) as *mut Self;
        unsafe { Box::from_raw(p) }
    }
}

pub trait Coordinate<T> {
    const MAX: usize;
    fn to_coord(state: &T) -> usize;
    fn from_coord(coord: usize) -> T;
}

pub struct EO;

impl Coordinate<Cube> for EO {
    const MAX: usize = 2usize.pow(11);

    fn to_coord(state: &Cube) -> usize {
        state
            .eo
            .iter()
            .take(11)
            .fold(0, |acc, &eo| 2 * acc + eo as usize)
    }

    fn from_coord(coord: usize) -> Cube {
        debug_assert!(coord < 2usize.pow(11), "number {coord} out of bounds");

        let mut digits = [0; 12];
        let mut n = coord;
        for i in (0..11).rev() {
            digits[i] = (n % 2) as u8;
            n /= 2;
        }
        digits[11] = (2 - digits.iter().take(11).sum::<u8>() % 2) % 2;

        let mut cube = Cube::solved();
        cube.eo = digits;
        cube
    }
}

pub struct CornerOrientation;
pub struct CornerPermutation;

impl Coordinate<Cube> for CornerOrientation {
    const MAX: usize = 2187;

    fn to_coord(state: &Cube) -> usize {
        state
            .co
            .iter()
            .take(7)
            .fold(0, |acc, &co| 3 * acc + co as usize)
    }

    fn from_coord(coord: usize) -> Cube {
        debug_assert!(coord < Self::MAX, "number {coord} out of bounds");

        let mut orientation = [0; 8];
        let mut n = coord;
        for i in (0..7).rev() {
            orientation[i] = (n % 3) as u8;
            n /= 3;
        }
        let sum = orientation.iter().take(7).sum::<u8>();
        orientation[7] = (3 - sum % 3) % 3;

        let mut cube = Cube::solved();
        cube.co = orientation;

        cube
    }
}

impl Coordinate<Cube> for CornerPermutation {
    const MAX: usize = 40320;

    fn to_coord(state: &Cube) -> usize {
        let mut x = 0;
        for i in (1..8).rev() {
            let mut s = 0;
            for j in (0..i).rev() {
                if state.cp[j] > state.cp[i] {
                    s += 1;
                }
            }
            x = (x + s) * i;
        }
        x
    }

    fn from_coord(coord: usize) -> Cube {
        const FACTORIALS: [usize; 8] = [0, 1, 2, 6, 24, 120, 720, 5040];

        debug_assert!(coord < 40320, "number {coord} out of bounds");

        let mut lehmer_code = [0; 8];
        let mut n = coord;
        for i in (1..8).rev() {
            let digit = n / FACTORIALS[i];

            debug_assert!(digit <= i, "{digit} should be <= {i}");

            lehmer_code[i] = digit as u8;
            n %= FACTORIALS[i];
        }

        let mut cp = [0; 8];
        let mut remaining: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7];

        for i in (0..=7).rev() {
            let digit = lehmer_code[i];
            let index = remaining[i - digit as usize];
            cp[i] = index;
            remaining.remove(i - digit as usize);
        }

        let mut cube = Cube::solved();
        cube.cp = cp;
        cube
    }
}

/*
impl<S, C0, C1> Coordinate<S> for (C0, C1)
where
    C0: Coordinate<S>,
    C1: Coordinate<S>,
{
    const MAX: usize = C0::MAX * C1::MAX;

    fn to_coord(state: &S) -> usize {
        C0::to_coord(state) * C1::MAX + C1::to_coord(state)
    }

    fn from_coord(coord: usize) -> S {
        let c0 = coord / C1::MAX;
        let c1 = coord % C1::MAX;
        (C0::from_coord(c0), C1::from_coord(c1))
    }
}
*/
impl Coordinate<Cube> for (CornerOrientation, CornerPermutation) {
    const MAX: usize = CornerOrientation::MAX * CornerPermutation::MAX;

    fn to_coord(state: &Cube) -> usize {
        CornerOrientation::to_coord(state) * CornerPermutation::MAX
            + CornerPermutation::to_coord(state)
    }

    fn from_coord(coord: usize) -> Cube {
        let c0 = coord / CornerPermutation::MAX;
        let c1 = coord % CornerPermutation::MAX;
        let (mut co, cp) = (
            CornerOrientation::from_coord(c0),
            CornerPermutation::from_coord(c1),
        );

        co.cp = cp.cp;
        co
    }
}

impl<const LOW: usize, const HIGH: usize> Coordinate<Cube>
    for (
        PartialEdgeOrientation<LOW, HIGH>,
        PartialEdgePermutation<LOW, HIGH>,
    )
where
    PartialEdgeOrientation<LOW, HIGH>: Coordinate<Cube>,
    PartialEdgePermutation<LOW, HIGH>: Coordinate<Cube>,
{
    const MAX: usize =
        PartialEdgeOrientation::<LOW, HIGH>::MAX * PartialEdgePermutation::<LOW, HIGH>::MAX;

    fn to_coord(state: &Cube) -> usize {
        PartialEdgeOrientation::<LOW, HIGH>::to_coord(state)
            * PartialEdgePermutation::<LOW, HIGH>::MAX
            + PartialEdgePermutation::<LOW, HIGH>::to_coord(state)
    }

    fn from_coord(coord: usize) -> Cube {
        let c0 = coord / PartialEdgePermutation::<LOW, HIGH>::MAX;
        let c1 = coord % PartialEdgePermutation::<LOW, HIGH>::MAX;
        let (mut eo, ep) = (
            PartialEdgeOrientation::<LOW, HIGH>::from_coord(c0),
            PartialEdgePermutation::<LOW, HIGH>::from_coord(c1),
        );

        eo.ep = ep.ep;
        eo
    }
}

pub struct PartialEdges<const LOW: usize, const HIGH: usize>;

impl<const LOW: usize, const HIGH: usize> PartialEdges<LOW, HIGH> {
    const SIZE: usize = HIGH - LOW;
    const ORIENTATION_SIZE: usize = 2usize.pow((Self::SIZE) as u32);
    const PERMUTATION_SIZE: usize = {
        let arrangements = factorial(Self::SIZE);
        let choices = choose(12, Self::SIZE);
        arrangements * choices
    };
}
impl<const LOW: usize, const HIGH: usize> Coordinate<Cube> for PartialEdges<LOW, HIGH>
where
    [(); Self::SIZE]: Sized,
{
    const MAX: usize = Self::PERMUTATION_SIZE * Self::ORIENTATION_SIZE;

    fn to_coord(state: &Cube) -> usize {
        debug_assert!(LOW < HIGH, "LOW must be < HIGH");
        debug_assert!(HIGH <= 12, "HIGH must be <= 12");

        let mut positions = [0; Self::SIZE];

        for (i, &edge) in state.ep.iter().enumerate() {
            if (LOW..HIGH).contains(&(edge as usize)) {
                positions[edge as usize - LOW] = i;
            }
        }

        let eo_coord = positions
            .iter()
            .fold(0, |acc, &i| 2 * acc + state.eo[i] as usize);

        // PartialEdgePermutation
        let mut lehmer = [0usize; Self::SIZE];
        for (i, &epi) in positions.iter().enumerate() {
            let s = positions.into_iter().take(i).filter(|&x| x > epi).count();
            lehmer[i] = s;
        }

        let arrangement: usize = lehmer
            .iter()
            .enumerate()
            .skip(1)
            .rev()
            .fold(0, |acc, (i, cur)| (acc + cur) * i);

        positions.sort();

        let choice: usize = positions
            .iter()
            .enumerate()
            .map(|(k, &c)| choose(c as usize, k + 1))
            .sum();

        let ep_coord = arrangement * const { choose(12, Self::SIZE) } + choice;

        eo_coord * Self::PERMUTATION_SIZE + ep_coord
    }

    fn from_coord(coord: usize) -> Cube {
        debug_assert!(LOW < HIGH, "LOW must be < HIGH");
        debug_assert!(HIGH <= 12, "HIGH must be <= 12");
        debug_assert!(coord < Self::MAX, "number {coord} out of bounds");
        // split the 2 coords
        let eo_coord = coord / Self::PERMUTATION_SIZE;
        let ep_coord = coord % Self::PERMUTATION_SIZE;

        // permutation
        let max = const { choose(12, Self::SIZE) };
        let arrangement = ep_coord / max;
        let mut choice = ep_coord % max;

        let mut remaining = Vec::new();

        for k in (1..=Self::SIZE).rev() {
            let (i, x) = greatest_combination(choice, k);
            remaining.push(i);
            choice -= x;
        }

        remaining.reverse();

        let mut lehmer_code = [0; Self::SIZE];
        let mut n = arrangement;
        for i in (1..Self::SIZE).rev() {
            let digit = n / factorial(i);

            debug_assert!(digit <= i, "{digit} should be <= {i}");

            lehmer_code[i] = digit as u8;
            n %= factorial(i);
        }

        let mut positions = [0; Self::SIZE];
        for i in (0..Self::SIZE).rev() {
            let digit = lehmer_code[i];
            let epi = remaining.remove(i - digit as usize);
            positions[i] = epi;
        }

        let mut remaining = Vec::from_iter((0..LOW).chain(HIGH..12));

        let mut ep = [None; 12];
        for (i, epi) in positions.into_iter().enumerate() {
            ep[epi as usize] = Some((i + LOW) as u8);
        }

        let ep = ep.map(|x| match x {
            Some(x) => x,
            None => remaining.pop().unwrap() as u8,
        });

        // edges
        let mut eo = [0; 12];
        let mut n = eo_coord;
        for i in (0..Self::SIZE).rev() {
            eo[positions[i] as usize] = (n % 2) as u8;
            n /= 2;
        }

        let mut cube = Cube::solved();
        cube.ep = ep;
        cube.eo = eo;
        cube
    }
}

pub struct PartialEdgeOrientation<const LOW: usize, const HIGH: usize>;

impl<const LOW: usize, const HIGH: usize> Coordinate<Cube> for PartialEdgeOrientation<LOW, HIGH> {
    const MAX: usize = 2usize.pow((HIGH - LOW) as u32);

    fn to_coord(state: &Cube) -> usize {
        debug_assert!(LOW < HIGH, "LOW must be < HIGH");
        debug_assert!(HIGH <= 12, "HIGH must be <= 12");

        state.eo[LOW..HIGH]
            .iter()
            .fold(0, |acc, &co| 2 * acc + co as usize)
    }

    fn from_coord(coord: usize) -> Cube {
        debug_assert!(LOW < HIGH, "LOW must be < HIGH");
        debug_assert!(HIGH <= 12, "HIGH must be <= 12");
        debug_assert!(coord < Self::MAX, "number {coord} out of bounds");

        let mut orientation = [0; 12];
        let mut n = coord;
        for i in (LOW..HIGH).rev() {
            orientation[i] = (n % 2) as u8;
            n /= 2;
        }

        let mut cube = Cube::solved();
        cube.eo = orientation;

        cube
    }
}

const fn factorial(mut n: usize) -> usize {
    let mut total = 1;
    while n > 0 {
        total *= n;
        n -= 1;
    }
    total
}

/// number of ways to choose k things from a set of n
const fn choose(n: usize, k: usize) -> usize {
    if k > n {
        0
    } else {
        // this could be optimised
        factorial(n) / (factorial(k) * factorial(n - k))
    }
}

fn greatest_combination(n: usize, k: usize) -> (u8, usize) {
    let mut prev = 0;
    let mut x = 0;

    loop {
        let choice = choose(x, k);
        if choice > n {
            return ((x - 1) as u8, prev);
        }
        prev = choice;
        x += 1;
    }
}

pub struct PartialEdgePermutation<const LOW: usize, const HIGH: usize>;
impl<const LOW: usize, const HIGH: usize> Coordinate<Cube> for PartialEdgePermutation<LOW, HIGH>
where
    [u8; HIGH - LOW]: Sized,
{
    const MAX: usize = {
        let k = HIGH - LOW;
        let arrangements = factorial(k);
        let choices = choose(12, k);
        arrangements * choices
    };

    fn to_coord(state: &Cube) -> usize {
        debug_assert!(LOW < HIGH, "LOW ({LOW}) should be < HIGH ({HIGH})");
        debug_assert!(HIGH <= 12, "HIGH ({HIGH}) should be <= 12");

        let mut lehmer = [0usize; HIGH - LOW];
        for i in 0..HIGH - LOW {
            let mut s = 0;
            for j in 0..i {
                if state.ep[LOW + j] > state.ep[LOW + i] {
                    s += 1;
                }
            }
            lehmer[i] = s;
        }

        let arrangement: usize = lehmer
            .iter()
            .enumerate()
            .skip(1)
            .rev()
            .fold(0, |acc, (i, cur)| (acc + cur) * i);

        let mut edges = Vec::from_iter(state.ep[LOW..HIGH].iter().copied());
        edges.sort();

        let choice: usize = zip(edges, 1..=HIGH - LOW)
            .map(|(c, k)| choose(c as usize, k))
            .sum();

        let coord = arrangement * const { choose(12, HIGH - LOW) } + choice;
        coord
    }

    fn from_coord(coord: usize) -> Cube {
        debug_assert!(LOW < HIGH, "LOW ({LOW}) should be < HIGH ({HIGH})");
        debug_assert!(HIGH <= 12, "HIGH ({HIGH}) should be <= 12");
        debug_assert!(coord < Self::MAX, "number {coord} out of bounds");

        let max = const { choose(12, HIGH - LOW) };
        let arrangement = coord / max;
        let mut choice = coord % max;

        let mut remaining = Vec::new();

        for k in (1..=HIGH - LOW).rev() {
            let (i, x) = greatest_combination(choice, k);
            remaining.push(i);
            choice -= x;
        }

        remaining.reverse();

        let mut lehmer_code = [0; HIGH - LOW];
        let mut n = arrangement;
        for i in (1..HIGH - LOW).rev() {
            let digit = n / factorial(i);

            debug_assert!(digit <= i, "{digit} should be <= {i}");

            lehmer_code[i] = digit as u8;
            n %= factorial(i);
        }

        let mut cube = Cube::solved();
        for i in (0..HIGH - LOW).rev() {
            let digit = lehmer_code[i];
            let epi = remaining.remove(i - digit as usize);
            cube.ep[LOW + i] = epi;
        }
        cube
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::Cube;
    use crate::mv::Move::*;

    #[test]
    fn corners_solved() {
        let solved = Cube::solved();
        assert_eq!(
            (
                CornerPermutation::to_coord(&solved),
                CornerOrientation::to_coord(&solved)
            ),
            (0, 0)
        );
    }

    #[test]
    fn corner_coordinates() {
        let scramble = R * U * U * F * L * B;

        assert_eq!(
            (
                CornerPermutation::to_coord(&scramble),
                CornerOrientation::to_coord(&scramble)
            ),
            (4467, 2050)
        );
    }

    #[test]
    fn corner_permutation_round_trip() {
        let scramble = R * U * U * F * L * B;

        let index = CornerPermutation::to_coord(&scramble);
        let reconstructed_cube = CornerPermutation::from_coord(index);

        assert_eq!(reconstructed_cube.cp, scramble.cp);
    }

    #[test]
    fn corner_orientation_round_trip() {
        let scramble = R * U * U * F * L * B;

        let index = CornerOrientation::to_coord(&scramble);
        let reconstructed_cube = CornerOrientation::from_coord(index);

        assert_eq!(reconstructed_cube.co, scramble.co);
    }

    #[test]
    fn choose_test() {
        for (n, expected) in (4..11).zip([0, 1, 6, 21, 56, 126, 252]) {
            assert_eq!(choose(n, 5), expected)
        }
    }

    #[test]
    fn test_greatest_combination_k5_n72() {
        // For k = 5, n = 72, find the largest x such that choose(x, 5) <= 72
        let (x, value) = greatest_combination(72, 5);
        // choose(7, 5) = 21, choose(8, 5) = 56, choose(9, 5) = 126
        // So, choose(8, 5) = 56 is the largest <= 72, so x should be 8, value 56
        assert_eq!(x, 8);
        assert_eq!(value, 56);
    }

    #[test]
    fn partial_edge_orientation_round_trip() {
        let scramble = R * U * U * F * L * B;
        {
            const LOW: usize = 0;
            const HIGH: usize = 6;
            let index = PartialEdgeOrientation::<LOW, HIGH>::to_coord(&scramble);
            let reconstructed_cube = PartialEdgeOrientation::<LOW, HIGH>::from_coord(index);

            assert_eq!(reconstructed_cube.eo[LOW..HIGH], scramble.eo[LOW..HIGH]);
        }

        {
            const LOW: usize = 6;
            const HIGH: usize = 12;
            let index = PartialEdgeOrientation::<LOW, HIGH>::to_coord(&scramble);
            let reconstructed_cube = PartialEdgeOrientation::<LOW, HIGH>::from_coord(index);

            assert_eq!(reconstructed_cube.eo[LOW..HIGH], scramble.eo[LOW..HIGH]);
        }
    }

    #[test]
    fn partial_edge_permutation_round_trip() {
        let scramble = R * U * U * F * L * B;

        {
            const LOW: usize = 0;
            const HIGH: usize = 6;
            let index = PartialEdgePermutation::<LOW, HIGH>::to_coord(&scramble);

            let reconstructed_cube = PartialEdgePermutation::<LOW, HIGH>::from_coord(index);

            assert_eq!(reconstructed_cube.ep[LOW..HIGH], scramble.ep[LOW..HIGH]);
        }
        {
            const LOW: usize = 6;
            const HIGH: usize = 12;
            let index = PartialEdgePermutation::<LOW, HIGH>::to_coord(&scramble);
            let reconstructed_cube = PartialEdgePermutation::<LOW, HIGH>::from_coord(index);

            assert_eq!(reconstructed_cube.ep[LOW..HIGH], scramble.ep[LOW..HIGH]);
        }
    }

    #[test]
    fn partial_edges_round_trip() {
        {
            const LOW: usize = 0;
            const HIGH: usize = 6;
            const COORDINATE: usize = 6969420;
            let state = PartialEdges::<LOW, HIGH>::from_coord(COORDINATE);

            let reconstructed_coordinate = PartialEdges::<LOW, HIGH>::to_coord(&state);

            assert_eq!(COORDINATE, reconstructed_coordinate);
        }
    }
}
