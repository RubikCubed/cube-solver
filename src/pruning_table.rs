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
}
