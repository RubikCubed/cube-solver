use colored::Colorize;

#[derive(Debug)]

pub struct Cube {
    eo: [u8; 12], // all <2
    ep: [u8; 12], // all unique, all <2
    co: [u8; 8],  // all <3
    cp: [u8; 8],  // all unique, all <8
}

pub enum FaceletAssociation {
    Center(Color),
    Edge(u8, u8),   // EP, EO
    Corner(u8, u8), // CP, CO
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}

fn edge_colors(edge: u8) -> &'static [Color; 2] {
    use Color::*;

    match edge {
        0 => &[White, Blue],
        1 => &[White, Red],
        2 => &[White, Green],
        3 => &[White, Orange],
        4 => &[Blue, Orange],
        5 => &[Blue, Red],
        6 => &[Green, Red],
        7 => &[Green, Orange],
        8 => &[Yellow, Blue],
        9 => &[Yellow, Red],
        10 => &[Yellow, Green],
        11 => &[Yellow, Orange],
        _ => panic!("Invalid edge number: {}", edge),
    }
}

fn corner_colors(corner: u8) -> &'static [Color; 3] {
    use Color::*;

    match corner {
        0 => &[White, Orange, Blue],
        1 => &[White, Blue, Red],
        2 => &[White, Red, Green],
        3 => &[White, Green, Orange],
        4 => &[Yellow, Blue, Orange],
        5 => &[Yellow, Red, Blue],
        6 => &[Yellow, Green, Red],
        7 => &[Yellow, Orange, Green],
        _ => panic!("Invalid corner number: {}", corner),
    }
}

// converts a facelet id into a cubie + orientation
fn associate_facelet(facelet: u8) -> FaceletAssociation {
    use FaceletAssociation::*;

    match facelet {
        // top / white
        0 => Corner(0, 0),
        1 => Edge(0, 0),
        2 => Corner(1, 0),
        3 => Edge(3, 0),
        4 => Center(Color::White),
        5 => Edge(1, 0),
        6 => Corner(3, 0),
        7 => Edge(2, 0),
        8 => Corner(2, 0),
        // top row
        // orange
        9 => Corner(0, 1),
        10 => Edge(3, 1),
        11 => Corner(3, 2),
        // green
        12 => Corner(3, 1),
        13 => Edge(2, 1),
        14 => Corner(2, 2),
        // red
        15 => Corner(2, 1),
        16 => Edge(1, 1),
        17 => Corner(1, 2),
        // blue
        18 => Corner(1, 1),
        19 => Edge(0, 1),
        20 => Corner(0, 2),
        // middle row
        // orange
        21 => Edge(4, 1),
        22 => Center(Color::Orange),
        23 => Edge(7, 1),
        // green
        24 => Edge(7, 0),
        25 => Center(Color::Green),
        26 => Edge(6, 0),
        // red
        27 => Edge(6, 1),
        28 => Center(Color::Red),
        29 => Edge(5, 1),
        // blue
        30 => Edge(5, 0),
        31 => Center(Color::Blue),
        32 => Edge(4, 0),
        // bottom row
        // orange
        33 => Corner(4, 2),
        34 => Edge(11, 1),
        35 => Corner(7, 1),
        // green
        36 => Corner(7, 2),
        37 => Edge(10, 1),
        38 => Corner(6, 1),
        // red
        39 => Corner(6, 2),
        40 => Edge(9, 1),
        41 => Corner(5, 1),
        // blue
        42 => Corner(5, 2),
        43 => Edge(8, 1),
        44 => Corner(4, 1),
        // bottom / yellow
        45 => Corner(7, 0),
        46 => Edge(10, 0),
        47 => Corner(6, 0),
        48 => Edge(11, 0),
        49 => Center(Color::Yellow),
        50 => Edge(9, 0),
        51 => Corner(4, 0),
        52 => Edge(8, 0),
        53 => Corner(5, 0),
        _ => panic!("Invalid Facelet {}", facelet),
    }
}

// predefined states
pub const SOLVED: Cube = Cube {
    eo: [0; 12],
    ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [0, 1, 2, 3, 4, 5, 6, 7],
};

// rotate the top face clockwise
pub const U: Cube = Cube {
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [3, 0, 1, 2, 4, 5, 6, 7],
};

// rotate the right face clockwise
pub const R: Cube = Cube {
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [0, 6, 2, 3, 4, 1, 9, 7, 8, 5, 10, 11],
    co: [0, 2, 1, 0, 0, 1, 2, 0],
    cp: [0, 2, 6, 3, 4, 1, 5, 7],
};

pub const Rprime: Cube = Cube {
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [0, 5, 2, 3, 4, 9, 1, 7, 8, 6, 10, 11],
    co: [0, 2, 1, 0, 0, 1, 2, 0],
    cp: [0, 5, 1, 3, 4, 6, 2, 7],
};

pub const L: Cube = Cube {
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [0, 1, 2, 4, 11, 5, 6, 3, 8, 9, 10, 7],
    co: [1, 0, 0, 2, 2, 0, 0, 1],
    cp: [4, 1, 2, 0, 7, 5, 6, 3],
};

// untested
pub const D: Cube = Cube {
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [0, 1, 2, 3, 5, 6, 7, 4],
};

// untested
pub const F: Cube = Cube {
    eo: [0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0],
    ep: [0, 1, 7, 3, 4, 5, 2, 10, 8, 9, 6, 11],
    co: [0, 0, 2, 1, 0, 0, 1, 2],
    cp: [0, 1, 3, 7, 4, 5, 2, 6],
};

pub const B: Cube = Cube {
    eo: [1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0],
    ep: [5, 1, 2, 3, 0, 8, 6, 7, 4, 9, 10, 11],
    co: [2, 1, 0, 0, 1, 2, 0, 0],
    cp: [1, 5, 2, 3, 0, 4, 6, 7],
};

impl Cube {
    pub fn apply(&self, mv: &Self) -> Self {
        Cube {
            ep: std::array::from_fn(|i| self.ep[mv.ep[i] as usize]),
            eo: std::array::from_fn(|i| (self.eo[mv.ep[i] as usize] + mv.eo[i]) % 2),
            cp: std::array::from_fn(|i| self.cp[mv.cp[i] as usize]),
            co: std::array::from_fn(|i| (self.co[mv.cp[i] as usize] + mv.co[i]) % 3),
        }
    }

    pub fn print_net(&self) {
        for (i, c) in self.to_facelets().iter().enumerate() {
            match i {
                0 | 3 | 6 | 45 | 48 | 51 => print!("\n      {}", c.tile()),
                9 | 21 | 33 => print!("\n{}", c.tile()),
                _ => print!("{}", c.tile()),
            }
        }
        println!()
    }

    pub fn to_facelets(&self) -> [Color; 54] {
        std::array::from_fn(|i| associate_facelet(i as u8).to_color(self))
    }
}

impl FaceletAssociation {
    fn to_color(&self, cube: &Cube) -> Color {
        match *self {
            FaceletAssociation::Corner(cp, co) => {
                let cpi = cube.cp[cp as usize];
                let coi = ((co + cube.co[cp as usize]) % 3) as usize;
                corner_colors(cpi)[coi]
            }
            FaceletAssociation::Edge(ep, eo) => {
                let epi = cube.ep[ep as usize];
                let eoi = ((eo + cube.eo[ep as usize]) % 2) as usize;
                edge_colors(epi)[eoi]
            }
            FaceletAssociation::Center(color) => color,
        }
    }
}

impl Color {
    pub fn tile(&self) -> colored::ColoredString {
        match &self {
            Color::White => "██".white(),
            Color::Yellow => "██".yellow(),
            Color::Red => "██".red(),
            Color::Orange => "██".custom_color((255, 128, 0)),
            Color::Blue => "██".blue(),
            Color::Green => "██".green(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        B,
        Color::{self, *},
        D, F, L, R, SOLVED, U,
    };

    const SOLVED_COLORS: [Color; 54] = [
        White, White, White, White, White, White, White, White, White, Orange, Orange, Orange,
        Green, Green, Green, Red, Red, Red, Blue, Blue, Blue, Orange, Orange, Orange, Green, Green,
        Green, Red, Red, Red, Blue, Blue, Blue, Orange, Orange, Orange, Green, Green, Green, Red,
        Red, Red, Blue, Blue, Blue, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow,
        Yellow,
    ];

    const L_COLORS: [Color; 54] = [
        Blue, White, White, Blue, White, White, Blue, White, White, Orange, Orange, Orange, White,
        Green, Green, Red, Red, Red, Blue, Blue, Yellow, Orange, Orange, Orange, White, Green,
        Green, Red, Red, Red, Blue, Blue, Yellow, Orange, Orange, Orange, White, Green, Green, Red,
        Red, Red, Blue, Blue, Yellow, Green, Yellow, Yellow, Green, Yellow, Yellow, Green, Yellow,
        Yellow,
    ];

    const R_COLORS: [Color; 54] = [
        White, White, Green, White, White, Green, White, White, Green, Orange, Orange, Orange,
        Green, Green, Yellow, Red, Red, Red, White, Blue, Blue, Orange, Orange, Orange, Green,
        Green, Yellow, Red, Red, Red, White, Blue, Blue, Orange, Orange, Orange, Green, Green,
        Yellow, Red, Red, Red, White, Blue, Blue, Yellow, Yellow, Blue, Yellow, Yellow, Blue,
        Yellow, Yellow, Blue,
    ];

    const U_COLORS: [Color; 54] = [
        White, White, White, White, White, White, White, White, White, Green, Green, Green, Red,
        Red, Red, Blue, Blue, Blue, Orange, Orange, Orange, Orange, Orange, Orange, Green, Green,
        Green, Red, Red, Red, Blue, Blue, Blue, Orange, Orange, Orange, Green, Green, Green, Red,
        Red, Red, Blue, Blue, Blue, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow,
        Yellow,
    ];

    const D_COLORS: [Color; 54] = [
        White, White, White, White, White, White, White, White, White, Orange, Orange, Orange,
        Green, Green, Green, Red, Red, Red, Blue, Blue, Blue, Orange, Orange, Orange, Green, Green,
        Green, Red, Red, Red, Blue, Blue, Blue, Blue, Blue, Blue, Orange, Orange, Orange, Green,
        Green, Green, Red, Red, Red, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow,
        Yellow, Yellow,
    ];

    const F_COLORS: [Color; 54] = [
        White, White, White, White, White, White, Orange, Orange, Orange, Orange, Orange, Yellow,
        Green, Green, Green, White, Red, Red, Blue, Blue, Blue, Orange, Orange, Yellow, Green,
        Green, Green, White, Red, Red, Blue, Blue, Blue, Orange, Orange, Yellow, Green, Green,
        Green, White, Red, Red, Blue, Blue, Blue, Red, Red, Red, Yellow, Yellow, Yellow, Yellow,
        Yellow, Yellow,
    ];

    const B_COLORS: [Color; 54] = [
        Red, Red, Red, White, White, White, White, White, White, White, Orange, Orange, Green,
        Green, Green, Red, Red, Yellow, Blue, Blue, Blue, White, Orange, Orange, Green, Green,
        Green, Red, Red, Yellow, Blue, Blue, Blue, White, Orange, Orange, Green, Green, Green, Red,
        Red, Yellow, Blue, Blue, Blue, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow, Orange,
        Orange, Orange,
    ];

    #[test]
    fn solved() {
        assert_eq!(SOLVED.to_facelets(), SOLVED_COLORS);
    }

    #[test]
    fn l() {
        assert_eq!(SOLVED.apply(&L).to_facelets(), L_COLORS);
    }

    #[test]
    fn r() {
        assert_eq!(SOLVED.apply(&R).to_facelets(), R_COLORS);
    }

    #[test]
    fn u() {
        assert_eq!(SOLVED.apply(&U).to_facelets(), U_COLORS);
    }

    #[test]
    fn d() {
        assert_eq!(SOLVED.apply(&D).to_facelets(), D_COLORS);
    }

    #[test]
    fn f() {
        assert_eq!(SOLVED.apply(&F).to_facelets(), F_COLORS);
    }

    #[test]
    fn b() {
        assert_eq!(SOLVED.apply(&B).to_facelets(), B_COLORS);
    }

    #[test]
    fn full_r_rotation() {
        assert_eq!(
            SOLVED.apply(&R).apply(&R).apply(&R).apply(&R).to_facelets(),
            SOLVED_COLORS
        )
    }
}
