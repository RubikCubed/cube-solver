use crate::{
    cube::{Cube, co_from_coord, cp_from_coord},
    puzzle::Puzzle,
};

pub trait Heuristic<T>: Copy {
    fn lower_bound(self, state: &T) -> u8;
}

#[derive(Clone, Copy)]
pub struct ZeroBound;
impl Heuristic<Cube> for ZeroBound {
    fn lower_bound(self, _state: &Cube) -> u8 {
        0
    }
}

#[derive(Clone, Copy)]
pub struct EOBound;
impl Heuristic<Cube> for EOBound {
    fn lower_bound(self, state: &Cube) -> u8 {
        state.eo.iter().sum::<u8>() % 4
    }
}

const CORNER_SIZE: usize = 2187 * 40320;

#[repr(transparent)]
pub struct Corners([u8; CORNER_SIZE]);

impl Corners {
    pub fn coord(cube: &Cube) -> usize {
        cube.corner_orientation_coordinate() * 40320 + cube.corner_perm_coordinate()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn new(table: Box<[u8; CORNER_SIZE]>) -> Box<Self> {
        let p = Box::into_raw(table) as *mut Corners;
        unsafe { Box::from_raw(p) }
    }

    pub fn generate() -> Box<Self> {
        eprintln!("Generating corner pruning table...");
        let mut table: Box<[u8; CORNER_SIZE]> = vec![0; CORNER_SIZE].try_into().unwrap();

        let start = std::time::Instant::now();

        let mut total_filled = 0;

        // all the face turns can be set to 1
        for mv in crate::mv::Move::ALL {
            let new_state = mv.to_cube();
            let index = Corners::coord(&new_state);
            table[index] = 1;
            total_filled += 1;
        }

        'depth: for depth in 2..=11 {
            for index in 1..CORNER_SIZE {
                if total_filled >= CORNER_SIZE {
                    eprintln!("Filled all entries in the table, stopping at depth {depth}");
                    break 'depth;
                }
                let moves_to_solve = table[index];

                if moves_to_solve != depth - 1 {
                    continue;
                }

                // go from index to orientation and permutation states
                let (coi, cpi) = Corners::index_to_coords(index);
                let (co, cp) = (co_from_coord(coi), cp_from_coord(cpi));

                let cube = Cube {
                    co,
                    cp,
                    eo: [0; 12],
                    ep: [0; 12],
                };

                // apply all moves to the current state, update the new indexes if they aren't set
                for mv in crate::mv::Move::ALL {
                    let new_state = cube.apply(&mv.to_cube());
                    let new_index = Corners::coord(&new_state);
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
        eprintln!("time taken to generate corner lookup table: {:?}", elapsed);

        Corners::new(table)
    }

    pub fn index_to_coords(index: usize) -> (usize, usize) {
        debug_assert!(index < CORNER_SIZE, "index {index} out of bounds");

        let orientation = index / 40320;
        let permutation = index % 40320;
        (orientation, permutation)
    }
}

impl Heuristic<Cube> for &Corners {
    fn lower_bound(self, state: &Cube) -> u8 {
        let index = Corners::coord(state);
        self.0[index]
    }
}

#[rustfmt::skip]
impl<T, H0, H1> Heuristic<T> for (H0, H1) 
where 
    H0: Heuristic<T>,
    H1: Heuristic<T>,
{
    fn lower_bound(self, state: &T) -> u8 {
        let (h0, h1) = self;
        [
            h0.lower_bound(state),
            h1.lower_bound(state),
        ].into_iter().max().unwrap()
    }
}

#[rustfmt::skip]
impl<T, H0, H1, H2> Heuristic<T> for (H0, H1, H2) 
where 
    H0: Heuristic<T>,
    H1: Heuristic<T>,
    H2: Heuristic<T>,
{
    fn lower_bound(self, state: &T) -> u8 {
        let (h0, h1, h2) = self;
        [
            h0.lower_bound(state),
            h1.lower_bound(state),
            h2.lower_bound(state),
        ].into_iter().max().unwrap()
    }
}

#[rustfmt::skip]
impl<T, H0, H1, H2, H3> Heuristic<T> for (H0, H1, H2, H3) 
where 
    H0: Heuristic<T>,
    H1: Heuristic<T>,
    H2: Heuristic<T>,
    H3: Heuristic<T>,
{
    fn lower_bound(self, state: &T) -> u8 {
        let (h0, h1, h2, h3) = self;
        [
            h0.lower_bound(state),
            h1.lower_bound(state),
            h2.lower_bound(state),
            h3.lower_bound(state),
        ].into_iter().max().unwrap()
    }
}
