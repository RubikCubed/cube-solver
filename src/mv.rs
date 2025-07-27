use super::cube::Cube;

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
        use moves::*;

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

    #[rustfmt::skip]
    pub fn redundant(self, other: Self) -> bool {
        use Move::*;
        match (self, other) {
              (U, U) | (U, U2) | (U, U3)
            | (U2, U) | (U2, U2) | (U2, U3)
            | (U3, U) | (U3, U2) | (U3, U3)
            | (R, R) | (R, R2) | (R, R3)
            | (R2, R) | (R2, R2) | (R2, R3)
            | (R3, R) | (R3, R2) | (R3, R3)
            | (F, F) | (F, F2) | (F, F3)
            | (F2, F) | (F2, F2) | (F2, F3)
            | (F3, F) | (F3, F2) | (F3, F3)
            | (L, L) | (L, L2) | (L, L3)
            | (L2, L) | (L2, L2) | (L2, L3)
            | (L3, L) | (L3, L2) | (L3, L3)
            | (D, D) | (D, D2) | (D, D3)
            | (D2, D) | (D2, D2) | (D2, D3)
            | (D3, D) | (D3, D2) | (D3, D3)
            | (B, B) | (B, B2) | (B, B3)
            | (B2, B) | (B2, B2) | (B2, B3)
            | (B3, B) | (B3, B2) | (B3, B3) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl std::ops::Mul<Move> for Move {
    type Output = Cube;

    fn mul(self, rhs: Move) -> Self::Output {
        self.to_cube() * rhs.to_cube()
    }
}

mod moves {
    use super::Cube;

    pub(super) const U: Cube = Cube {
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11],
        co: [0, 0, 0, 0, 0, 0, 0, 0],
        cp: [3, 0, 1, 2, 4, 5, 6, 7],
    };

    pub(super) const UPRIME: Cube = Cube {
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11],
        co: [0, 0, 0, 0, 0, 0, 0, 0],
        cp: [1, 2, 3, 0, 4, 5, 6, 7],
    };

    // rotate the right face clockwise
    pub(super) const R: Cube = Cube {
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [0, 6, 2, 3, 4, 1, 9, 7, 8, 5, 10, 11],
        co: [0, 2, 1, 0, 0, 1, 2, 0],
        cp: [0, 2, 6, 3, 4, 1, 5, 7],
    };

    pub(super) const RPRIME: Cube = Cube {
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [0, 5, 2, 3, 4, 9, 1, 7, 8, 6, 10, 11],
        co: [0, 2, 1, 0, 0, 1, 2, 0],
        cp: [0, 5, 1, 3, 4, 6, 2, 7],
    };

    pub(super) const L: Cube = Cube {
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [0, 1, 2, 4, 11, 5, 6, 3, 8, 9, 10, 7],
        co: [1, 0, 0, 2, 2, 0, 0, 1],
        cp: [4, 1, 2, 0, 7, 5, 6, 3],
    };

    pub(super) const LPRIME: Cube = Cube {
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [0, 1, 2, 7, 3, 5, 6, 11, 8, 9, 10, 4],
        co: [1, 0, 0, 2, 2, 0, 0, 1],
        cp: [3, 1, 2, 7, 0, 5, 6, 4],
    };

    pub(super) const D: Cube = Cube {
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8],
        co: [0, 0, 0, 0, 0, 0, 0, 0],
        cp: [0, 1, 2, 3, 5, 6, 7, 4],
    };

    pub(super) const DPRIME: Cube = Cube {
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [0, 1, 2, 3, 4, 5, 6, 7, 11, 8, 9, 10],
        co: [0, 0, 0, 0, 0, 0, 0, 0],
        cp: [0, 1, 2, 3, 7, 4, 5, 6],
    };

    pub(super) const F: Cube = Cube {
        eo: [0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0],
        ep: [0, 1, 7, 3, 4, 5, 2, 10, 8, 9, 6, 11],
        co: [0, 0, 2, 1, 0, 0, 1, 2],
        cp: [0, 1, 3, 7, 4, 5, 2, 6],
    };

    pub(super) const FPRIME: Cube = Cube {
        eo: [0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0],
        ep: [0, 1, 6, 3, 4, 5, 10, 2, 8, 9, 7, 11],
        co: [0, 0, 2, 1, 0, 0, 1, 2],
        cp: [0, 1, 6, 2, 4, 5, 7, 3],
    };

    pub(super) const B: Cube = Cube {
        eo: [1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0],
        ep: [5, 1, 2, 3, 0, 8, 6, 7, 4, 9, 10, 11],
        co: [2, 1, 0, 0, 1, 2, 0, 0],
        cp: [1, 5, 2, 3, 0, 4, 6, 7],
    };

    pub(super) const BPRIME: Cube = Cube {
        eo: [1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0],
        ep: [4, 1, 2, 3, 8, 0, 6, 7, 5, 9, 10, 11],
        co: [2, 1, 0, 0, 1, 2, 0, 0],
        cp: [4, 0, 2, 3, 5, 1, 6, 7],
    };

    pub(super) const U2: Cube = Cube {
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [2, 3, 0, 1, 4, 5, 6, 7, 8, 9, 10, 11],
        co: [0, 0, 0, 0, 0, 0, 0, 0],
        cp: [2, 3, 0, 1, 4, 5, 6, 7],
    };

    pub(super) const R2: Cube = Cube {
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [0, 9, 2, 3, 4, 6, 5, 7, 8, 1, 10, 11],
        co: [0, 0, 0, 0, 0, 0, 0, 0],
        cp: [0, 6, 5, 3, 4, 2, 1, 7],
    };

    pub(super) const L2: Cube = Cube {
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [0, 1, 2, 11, 7, 5, 6, 4, 8, 9, 10, 3],
        co: [0, 0, 0, 0, 0, 0, 0, 0],
        cp: [7, 1, 2, 4, 3, 5, 6, 0],
    };

    pub(super) const D2: Cube = Cube {
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [0, 1, 2, 3, 4, 5, 6, 7, 10, 11, 8, 9],
        co: [0, 0, 0, 0, 0, 0, 0, 0],
        cp: [0, 1, 2, 3, 6, 7, 4, 5],
    };

    pub(super) const F2: Cube = Cube {
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [0, 1, 10, 3, 4, 5, 7, 6, 8, 9, 2, 11],
        co: [0, 0, 0, 0, 0, 0, 0, 0],
        cp: [0, 1, 7, 6, 4, 5, 3, 2],
    };

    pub(super) const B2: Cube = Cube {
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ep: [8, 1, 2, 3, 5, 4, 6, 7, 0, 9, 10, 11],
        co: [0, 0, 0, 0, 0, 0, 0, 0],
        cp: [5, 4, 2, 3, 1, 0, 6, 7],
    };
}
