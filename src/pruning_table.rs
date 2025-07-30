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
