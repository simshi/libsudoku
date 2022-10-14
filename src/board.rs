use std::{fmt, sync::Once};

use crate::candidates::Candidates;

static INIT: Once = Once::new();
static mut PEERS: [[[(u8, u8); 20]; 9]; 9] = [[[(0, 0); 20]; 9]; 9];

/// Board of the game
///
/// A board has 9*9 cells
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Board([[Candidates; 9]; 9]);
impl Board {
    pub fn new(s: &str) -> Option<Self> {
        Self::must_init();

        let mut b: Self = Default::default();
        let mut i = 0;
        let mut j = 0;
        for c in s.chars() {
            if ('1'..='9').contains(&c) || c == '.' {
                b.0[i][j] = Candidates::from(c);
                if j < 8 {
                    j += 1;
                } else {
                    i += 1;
                    j = 0;
                }
            }
        }

        if i != 9 || j != 0 {
            // println!("invalid input:{},{},{};", i, j, b);
            None
        } else {
            Some(b)
        }
    }
    pub fn peers_of(row: usize, col: usize) -> &'static [(u8, u8); 20] {
        unsafe { &PEERS[row][col] }
    }
    pub fn cell(&self, i: usize, j: usize) -> Candidates {
        self.0[i][j]
    }
    pub fn set_cell(&mut self, i: usize, j: usize, cs: Candidates) {
        self.0[i][j] = cs;
    }
    pub fn is_done(&self, i: usize, j: usize) -> bool {
        self.0[i][j].is_done()
    }
    pub fn lucky(&self, i: usize, j: usize) -> char {
        self.0[i][j].lucky()
    }
    pub fn substract(&mut self, i: usize, j: usize, cs: Candidates) {
        self.0[i][j].substract(&cs)
    }
    pub fn iter(&self) -> std::slice::Iter<'_, [Candidates; 9]> {
        self.0.iter()
    }
    pub fn write_arr(&self, arr: &mut [[char; 9]; 9]) {
        for (p_line, b_line) in arr.iter_mut().zip(self.0.iter()) {
            for (p_char, cell) in p_line.iter_mut().zip(b_line.iter()) {
                // CAUTION: `to_string().chars().nth(0).unwrap()` takes 3000 ns...
                // *p_char = g_cell.to_string().chars().nth(0).unwrap();
                *p_char = cell.lucky();
            }
        }
    }

    fn must_init() {
        INIT.call_once(|| unsafe {
            for (row, line) in PEERS.iter_mut().enumerate() {
                for (col, cell_peers) in line.iter_mut().enumerate() {
                    let mut k = 0;
                    for i in 0..9 {
                        // same row
                        if i != col {
                            cell_peers[k] = (row as u8, i as u8);
                            k += 1;
                        }
                        // same col
                        if i != row {
                            cell_peers[k] = (i as u8, col as u8);
                            k += 1;
                        }
                    }
                    // same block
                    let (r_s, c_s) = (row / 3 * 3, col / 3 * 3);
                    for i in r_s..r_s + 3 {
                        for j in c_s..c_s + 3 {
                            if i != row && j != col {
                                PEERS[row][col][k] = (i as u8, j as u8);
                                k += 1;
                            }
                        }
                    }
                    PEERS[row][col].sort_unstable();
                }
            }
        });
    }
}

impl From<[[char; 9]; 9]> for Board {
    fn from(puzzle: [[char; 9]; 9]) -> Self {
        let mut b = Self::default();
        for (p_line, b_line) in puzzle.iter().zip(b.0.iter_mut()) {
            for (&p_char, b_cell) in p_line.iter().zip(b_line.iter_mut()) {
                *b_cell = Candidates::from(p_char);
            }
        }
        b
    }
}

impl fmt::Display for Board {
    /// display the Ripple as candidates list
    ///       <---> as width of col B
    ///   A     B    C      D     E    F      G    H    I
    /// +--------------------------------------------------+
    /// | 57   457   2   |   8   347   6   |  9    1    3  |
    /// |  8  45679  1   | 23457 3479 2357 | 346 3467  36  |
    /// |  3  4679   46  |  47   479   1   | 468   2    5  |
    /// +--------------------------------------------------+
    /// |  6  2578   9   |  37    1   378  | 238  358   4  |
    /// | 27  2478  348  |   6    5    9   | 238  38   138 |
    /// |  1   458  3458 |  34    2    38  |  7  3568   9  |
    /// +--------------------------------------------------+
    /// |  4    3   568  |   1   678  578  | 68   689   2  |
    /// | 259 25689 568  |  235  368  2358 |  1  34689  7  |
    /// |  2    1    7   |   9   368   4   |  5   368  368 |
    /// +--------------------------------------------------+
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let g = &self.0;
        // max width of each col, for alignment
        let cols_width = (0..9)
            .map(|col| (0..9).map(|row| g[row][col].len()).max().unwrap())
            .collect::<Vec<_>>();
        // total width with spaces
        let total_width = cols_width.iter().sum::<usize>() + 9 + 5;
        for (i, line) in g.iter().enumerate() {
            // block line
            if i % 3 == 0 {
                f.write_str(" +")?;
                for _ in 0..total_width {
                    f.write_str("-")?;
                }
                f.write_str("+\n")?;
            }
            for (j, col_width) in cols_width.iter().enumerate() {
                // block bound
                if j % 3 == 0 {
                    f.write_str(" |")?;
                }
                // max width of this col
                let w = col_width + 1;
                // spaces required for this cell, if len==0, will show as 'X'
                let spaces = w - std::cmp::max(1, line[j].len());
                // leading spaces, put one more as leading
                for _ in 0..((spaces + 1) / 2) {
                    f.write_str(" ")?
                }
                // candidates
                f.write_str(&line[j].to_string())?;
                // tail spaces
                for _ in 0..(spaces / 2) {
                    f.write_str(" ")?
                }
            }
            // block tail bound
            f.write_str(" |\n")?;
        }

        // block line
        f.write_str(" +")?;
        for _ in 0..total_width {
            f.write_str("-")?;
        }
        f.write_str("+\n")
    }
}
