use super::cube::{
    B, B2, BPRIME, Cube, D, D2, DPRIME, F, F2, FPRIME, L, L2, LPRIME, R, R2, RPRIME, U, U2, UPRIME,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[rustfmt::skip]
pub enum Move {
    U, U2, U3,
    D, D2, D3,
    L, L2, L3,
    R, R2, R3,
    F, F2, F3,
    B, B2, B3
}

impl Move {
    pub const ALL: &[Self] = {
        use Move::*;
        &[
            U, U2, U3, D, D2, D3, L, L2, L3, R, R2, R3, F, F2, F3, B, B2, B3,
        ]
    };

    pub fn to_cube(self) -> Cube {
        match self {
            Move::U => U,
            Move::U2 => U2,
            Move::U3 => UPRIME,
            Move::D => D,
            Move::D2 => D2,
            Move::D3 => DPRIME,
            Move::L => L,
            Move::L2 => L2,
            Move::L3 => LPRIME,
            Move::R => R,
            Move::R2 => R2,
            Move::R3 => RPRIME,
            Move::F => F,
            Move::F2 => F2,
            Move::F3 => FPRIME,
            Move::B => B,
            Move::B2 => B2,
            Move::B3 => BPRIME,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Move::U => "U",
            Move::U2 => "U2",
            Move::U3 => "U'",
            Move::D => "D",
            Move::D2 => "D2",
            Move::D3 => "D'",
            Move::L => "L",
            Move::L2 => "L2",
            Move::L3 => "L'",
            Move::R => "R",
            Move::R2 => "R2",
            Move::R3 => "R'",
            Move::F => "F",
            Move::F2 => "F2",
            Move::F3 => "F'",
            Move::B => "B",
            Move::B2 => "B2",
            Move::B3 => "B'",
        }
    }

    pub fn cancels(self, other: Self) -> bool {
        use Move::*;
        #[rustfmt::skip]
        match (self, other) {
              (U, U3) | (U3, U) | (U2, U2)
            | (D, D3) | (D3, D) | (D2, D2)
            | (L, L3) | (L3, L) | (L2, L2)
            | (R, R3) | (R3, R) | (R2, R2)
            | (F, F3) | (F3, F) | (F2, F2)
            | (B, B3) | (B3, B) | (B2, B2) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
