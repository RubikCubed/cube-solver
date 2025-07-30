use crate::cube::Cube;

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
