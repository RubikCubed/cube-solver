use colored::Colorize;

use super::heuristics::Heuristic;
use super::mv::Move;

#[derive(Debug, Clone, PartialEq)]
pub struct Cube {
    pub(crate) eo: [u8; 12], // all <2
    pub(crate) ep: [u8; 12], // all unique, all <2
    pub(crate) co: [u8; 8],  // all <3
    pub(crate) cp: [u8; 8],  // all unique, all <8
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

pub fn ida<T: Puzzle + Clone>(puzzle: T, max_depth: u8, h: impl Heuristic<T>) -> Option<Vec<Move>>
where
    for<'a> &'a T: std::ops::Mul<Move, Output = T>,
{
    for depth in 0..=max_depth {
        eprintln!("starting depth {depth}...");
        let start = std::time::Instant::now();
        let mut nodes = (0, 0);

        let puzzle = puzzle.clone();
        let path = dfs(0, Vec::new(), depth, puzzle, &mut nodes, h);
        let elapsed = start.elapsed();
        let (branches, leaves) = nodes;
        eprintln!(
            "searched {} nodes in {:.2?} at {:.2}M nodes/s, branching factor: {:.2}",
            branches + leaves,
            elapsed,
            (branches + leaves) as f64 / elapsed.as_secs_f64() / 1_000_000.0,
            if branches != 0 {
                (branches + leaves - 1) as f64 / branches as f64
            } else {
                0.0
            }
        );
        if let Some(path) = path {
            return Some(path);
        }
    }
    None
}

pub fn co_from_coord(number: usize) -> [u8; 8] {
    debug_assert!(number < 2187, "number {number} out of bounds");

    let mut digits = [0; 8];
    let mut n = number;
    for i in (0..7).rev() {
        digits[i] = (n % 3) as u8;
        n /= 3;
    }
    let sum = digits.iter().take(7).sum::<u8>();
    digits[7] = (3 - sum % 3) % 3;
    digits
}

static FACTORIALS: [usize; 8] = [0, 1, 2, 6, 24, 120, 720, 5040];

pub fn cp_from_coord(number: usize) -> [u8; 8] {
    debug_assert!(number < 40320, "number {number} out of bounds");

    let mut lehmer_code = [0; 8];
    let mut n = number;
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
    cp
}

use crate::puzzle::Puzzle;

impl Puzzle for Cube {
    fn solved() -> Self {
        SOLVED
    }

    fn is_solved(&self) -> bool {
        self == &Self::solved()
    }
}

pub fn dfs<T: Puzzle>(
    depth: u8,
    path: Vec<Move>,
    max_depth: u8,
    puzzle: T,
    nodes: &mut (u64, u64),
    h: impl Heuristic<T>,
) -> Option<Vec<Move>>
where
    for<'a> &'a T: std::ops::Mul<Move, Output = T>,
{
    if depth >= max_depth {
        nodes.1 += 1;
        if puzzle.is_solved() { Some(path) } else { None }
    } else if depth + h.lower_bound(&puzzle) > max_depth {
        None
    } else {
        if let [.., x, y] = &path[..] {
            if x.redundant(*y) {
                return None;
            }
        }

        nodes.0 += 1;

        Move::ALL.into_iter().find_map(|m| {
            let mut path = path.clone();
            path.push(*m);
            dfs(depth + 1, path, max_depth, &puzzle * *m, nodes, h)
        })
    }
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

pub const SUPERFLIP: Cube = Cube {
    eo: [1; 12],
    ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [0, 1, 2, 3, 4, 5, 6, 7],
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

    pub fn corner_perm_coordinate(&self) -> usize {
        let mut x = 0;
        for i in (1..8).rev() {
            let mut s = 0;
            for j in (0..i).rev() {
                if self.cp[j] > self.cp[i] {
                    s += 1;
                }
            }
            x = (x + s) * i;
        }
        x
    }

    pub fn corner_orientation_coordinate(&self) -> usize {
        self.co
            .iter()
            .take(7)
            .fold(0, |acc, &co| 3 * acc + co as usize)
    }
}

impl std::iter::Product for Cube {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(SOLVED, |acc, cur| acc.apply(&cur))
    }
}

impl std::ops::Mul for &Cube {
    type Output = Cube;

    fn mul(self, rhs: Self) -> Self::Output {
        self.apply(rhs)
    }
}

impl std::ops::Mul for Cube {
    type Output = Cube;

    fn mul(self, rhs: Self) -> Self::Output {
        self.apply(&rhs)
    }
}

impl std::ops::Mul<Move> for Cube {
    type Output = Cube;

    fn mul(self, rhs: Move) -> Self::Output {
        self.apply(&rhs.to_cube())
    }
}

impl std::ops::Mul<Move> for &Cube {
    type Output = Cube;

    fn mul(self, rhs: Move) -> Self::Output {
        self.apply(&rhs.to_cube())
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
        Color::{self, *},
        *,
    };
    use crate::mv::Move::*;

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
    fn identity() {
        assert_eq!(SOLVED * R, R.to_cube());
    }

    #[test]
    fn move_colors() {
        let pairs = [
            (U, U_COLORS),
            (D, D_COLORS),
            (L, L_COLORS),
            (R, R_COLORS),
            (F, F_COLORS),
            (B, B_COLORS),
        ];

        for (mv, colors) in pairs {
            assert_eq!(mv.to_cube().to_facelets(), colors);
        }
    }

    #[test]
    fn move_inversions() {
        let pairs = [(U, U3), (D, D3), (L, L3), (R, R3), (F, F3), (B, B3)];
        for (mv, mv3) in pairs {
            assert_eq!(mv.to_cube(), mv3 * mv3 * mv3);
        }
    }

    #[test]
    fn full_r_rotation() {
        assert_eq!(SOLVED * R * R2 * R, SOLVED)
    }

    #[test]
    fn superflip() {
        #[rustfmt::skip]
        let superflip_moves: Cube = U * R2 * F * B * R * B2 * R * U2 * L * B2 * R * U3 * D3 * R2 * F * R3 * L * B2 * U2 * F2;

        assert_eq!(superflip_moves.to_facelets(), SUPERFLIP.to_facelets())
    }

    #[test]
    fn corner_perms_solved() {
        assert_eq!(
            (
                SOLVED.corner_perm_coordinate(),
                SOLVED.corner_orientation_coordinate()
            ),
            (0, 0)
        );
    }

    #[test]
    fn corner_coordinates() {
        let scramble = R * U * U * F * L * B;

        assert_eq!(
            (
                scramble.corner_perm_coordinate(),
                scramble.corner_orientation_coordinate()
            ),
            (4467, 2050)
        );
    }

    #[test]
    fn index_to_coords() {
        use crate::heuristics::Corners;

        let scramble = R * U * U * F * L * B;
        let coords = (
            scramble.corner_orientation_coordinate(),
            scramble.corner_perm_coordinate(),
        );

        let index = Corners::coord(&scramble);

        assert_eq!(coords, Corners::index_to_coords(index));
    }

    #[test]
    fn coord_to_co() {
        let scramble = R * U * U * F * L * B;

        assert_eq!(
            scramble.co,
            co_from_coord(scramble.corner_orientation_coordinate())
        );
    }

    #[test]
    fn corner_permutation_round_trip() {
        use crate::heuristics::Corners;

        let scramble = R * U * U * F * L * B;

        let index = Corners::coord(&scramble);

        let (_, cpcoord) = Corners::index_to_coords(index);
        let cp_state: [u8; 8] = cp_from_coord(cpcoord);

        assert_eq!(cp_state, scramble.cp);
    }

    #[test]
    fn corner_orientation_round_trip() {
        use crate::heuristics::Corners;

        let scramble = R * U * U * F * L * B;

        let index = Corners::coord(&scramble);

        let (cocoord, _) = Corners::index_to_coords(index);
        let co_state: [u8; 8] = co_from_coord(cocoord);

        assert_eq!(co_state, scramble.co);
    }
}
