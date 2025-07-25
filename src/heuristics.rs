use super::cube::Cube;

pub trait Heuristic: Copy {
    fn lower_bound(self, state: &Cube) -> u8;
}

#[derive(Clone, Copy)]
pub struct ZeroBound;
impl Heuristic for ZeroBound {
    fn lower_bound(self, _state: &Cube) -> u8 {
        0
    }
}

#[derive(Clone, Copy)]
pub struct EOBound;
impl Heuristic for EOBound {
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

    pub fn generate() -> Box<Corners> {
        let table: Box<[_; CORNER_SIZE]> = vec![0; CORNER_SIZE].try_into().unwrap();

        // table.map(Corners)
        let p = Box::into_raw(table) as *mut Corners;
        unsafe { Box::from_raw(p) }
    }

    pub fn index_to_coords(index: usize) -> (usize, usize) {
        debug_assert!(index < CORNER_SIZE, "index {index} out of bounds");

        let orientation = index / 40320;
        let permutation = index % 40320;
        (orientation, permutation)
    }
}

impl Heuristic for &Corners {
    fn lower_bound(self, state: &Cube) -> u8 {
        let index = Corners::coord(state);
        self.0[index]
    }
}

#[rustfmt::skip]
impl<H0, H1> Heuristic for (H0, H1) 
where 
    H0: Heuristic,
    H1: Heuristic,
{
    fn lower_bound(self, state: &Cube) -> u8 {
        let (h0, h1) = self;
        [
            h0.lower_bound(state),
            h1.lower_bound(state),
        ].into_iter().max().unwrap()
    }
}

#[rustfmt::skip]
impl<H0, H1, H2> Heuristic for (H0, H1, H2) 
where 
    H0: Heuristic,
    H1: Heuristic,
    H2: Heuristic,
{
    fn lower_bound(self, state: &Cube) -> u8 {
        let (h0, h1, h2) = self;
        [
            h0.lower_bound(state),
            h1.lower_bound(state),
            h2.lower_bound(state),
        ].into_iter().max().unwrap()
    }
}

#[rustfmt::skip]
impl<H0, H1, H2, H3> Heuristic for (H0, H1, H2, H3) 
where 
    H0: Heuristic,
    H1: Heuristic,
    H2: Heuristic,
    H3: Heuristic,
{
    fn lower_bound(self, state: &Cube) -> u8 {
        let (h0, h1, h2, h3) = self;
        [
            h0.lower_bound(state),
            h1.lower_bound(state),
            h2.lower_bound(state),
            h3.lower_bound(state),
        ].into_iter().max().unwrap()
    }
}
