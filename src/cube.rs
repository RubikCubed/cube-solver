struct Cube {
  eo: [u8; 12], // all <2
  ep: [u8; 12], // all unique, all <2
  co: [u8; 8],  // all <3
  cp: [u8; 8],  // all unique, all <8
}

// eo = edge orientation
// ep = edge permutation

/*
top
  0
3  1
  2

middle
4  5
7  6

bottom
   8
11   9
  10
*/

/*
top
1 2
4 3

bottom
5 6
8 7
*/

pub const SOLVED: Cube = Cube {
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [0, 1, 2, 3, 4, 5, 6, 7]
};

// rotate the top face clockwise
pub const U: Cube = Cube {
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [3, 0, 1, 2, 4, 5, 6, 7]
};

// rotate the right face clockwise
pub const R: Cube = Cube {
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [0, 6, 2, 3, 4, 1, 9, 7, 8, 5, 10, 11],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [1, 3, 7, 4, 5, 2, 6, 8]
};

pub const L: Cube = Cube {
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [0, 1, 2, 4, 11, 5, 6, 3, 8, 9, 10, 7],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [4, 1, 2, 0, 8, 5, 6, 3]
};