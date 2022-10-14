use std::convert::From;
use std::fmt;

use crate::board::*;
use crate::candidates::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ripple {
    g: Board,
    // stats
    n_try: usize,
    n_triplex: usize,
}
struct Hint {
    first_unsolved_row: usize,
    first_unsolved_col: usize,
}
impl Ripple {
    pub fn new(s: &str) -> Option<Self> {
        Some(Self {
            g: Board::new(s)?,
            n_try: 0,
            n_triplex: 0,
        })
    }
    pub fn solve(s: &str) -> Option<String> {
        let mut b = Self {
            g: Board::new(s)?,
            n_try: 0,
            n_triplex: 0,
        };

        if b.do_solve() {
            let mut s = String::with_capacity(81);
            for i in 0..9 {
                for j in 0..9 {
                    // CAUTION: `to_string().chars().nth(0).unwrap()` takes 3000 ns...
                    // s.push(b.g[i][j].to_string().chars().nth(0).unwrap());
                    s.push(b.g.lucky(i, j));
                }
            }
            Some(s)
        } else {
            None
        }
    }
    pub fn solve_arr(puzzle: &mut [[char; 9]; 9]) -> bool {
        let mut b = Self {
            g: Board::from(*puzzle),
            n_try: 0,
            n_triplex: 0,
        };

        if b.do_solve() {
            b.g.write_arr(puzzle);
            true
        } else {
            false
        }
    }
    pub fn do_solve(&mut self) -> bool {
        // init
        for i in 0..9 {
            for j in 0..9 {
                if self.g.is_done(i, j) && !Self::ripple(&mut self.g, i, j) {
                    return false;
                }
            }
        }
        // println!("init done:\n{}", self);

        self.backtrack(&mut self.g.clone())
    }

    fn ripple(g: &mut Board, i: usize, j: usize) -> bool {
        let cs = g.cell(i, j);
        for &(row, col) in Board::peers_of(i, j) {
            let (row, col) = (row as usize, col as usize);
            if g.cell(row, col) == cs {
                return false;
            }
            if g.is_done(row, col) {
                continue;
            }
            g.substract(row, col, cs);
            if g.is_done(row, col) {
                // find a determined cell
                if !Self::ripple(g, row, col) {
                    return false;
                }
            }
        }
        true
    }
    fn backtrack(&mut self, g0: &mut Board) -> bool {
        let (row, col, ca, hint) = Self::next_least_unsolved(g0);
        // all cell is done, copy back the result to self.g
        if ca.is_done() {
            self.g = *g0;
            return true;
        }

        // try triplex first
        if !self.triplex(g0, hint) {
            return false;
        }

        // try on the current unsolved cell
        let mut g = *g0;
        for c in ca.iter() {
            self.n_try += 1;
            // make a guess
            g.set_cell(row, col, c);
            if Self::ripple(&mut g, row, col) && self.backtrack(&mut g) {
                return true;
            }
            // rollback
            g = *g0;
        }

        false
    }
    fn next_least_unsolved(g: &Board) -> (usize, usize, Candidates, Hint) {
        let mut min_row = 0;
        let mut min_col = 0;
        let mut min_len = 10;
        let mut hint = Hint {
            first_unsolved_row: 10,
            first_unsolved_col: 10,
        };
        for (row, line) in g.iter().enumerate() {
            for (col, cell) in line.iter().enumerate() {
                let ca = *cell;
                if ca.len() == 2 {
                    if row < hint.first_unsolved_row {
                        hint.first_unsolved_row = row;
                    }
                    if col < hint.first_unsolved_col {
                        hint.first_unsolved_col = col;
                    }

                    return (row, col, ca, hint);
                } else if ca.len() > 1 {
                    if row < hint.first_unsolved_row {
                        hint.first_unsolved_row = row;
                    }
                    if col < hint.first_unsolved_col {
                        hint.first_unsolved_col = col;
                    }

                    if ca.len() < min_len {
                        min_row = row;
                        min_col = col;
                        min_len = ca.len();
                    }
                }
            }
        }
        (min_row, min_col, g.cell(min_row, min_col), hint)
    }
    fn triplex(&mut self, b: &mut Board, hint: Hint) -> bool {
        for i in 0..9 {
            for g in (0..9).step_by(3) {
                // row
                if i >= hint.first_unsolved_row {
                    let (ca1, ca2, ca3) = (b.cell(i, g), b.cell(i, g + 1), b.cell(i, g + 2));
                    if !(ca1.is_done() || ca2.is_done() || ca3.is_done())
                        && ca1.len() <= 3
                        && ca2.len() <= 3
                        && ca3.len() <= 3
                    {
                        let uc = Candidates::union(ca1, ca2, ca3);
                        if uc.len() == 3 {
                            self.n_triplex += 1;
                            if !Self::triplex_ripple_row(b, i, g, uc)
                                || !Self::triplex_ripple_row_block(b, i, g, uc)
                            {
                                return false;
                            }
                        }
                    }
                }

                // col
                if i >= hint.first_unsolved_col {
                    let (ca1, ca2, ca3) = (b.cell(g, i), b.cell(g + 1, i), b.cell(g + 2, i));
                    if !(ca1.is_done() || ca2.is_done() || ca3.is_done())
                        && ca1.len() <= 3
                        && ca2.len() <= 3
                        && ca3.len() <= 3
                    {
                        let uc = Candidates::union(ca1, ca2, ca3);
                        if uc.len() == 3 {
                            self.n_triplex += 1;
                            if !Self::triplex_ripple_col(b, g, i, uc)
                                || !Self::triplex_ripple_col_block(b, g, i, uc)
                            {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }
    // weired, no big change in performance whether turns on/off row/col ripple
    fn triplex_ripple_row(b: &mut Board, row: usize, g: usize, uc: Candidates) -> bool {
        for c in 0..9 {
            if g <= c || c < g + 3 {
                continue;
            }
            if !Self::eliminate_multi(b, row, c, uc) {
                return false;
            }
        }
        true
    }
    fn triplex_ripple_col(b: &mut Board, row: usize, col: usize, uc: Candidates) -> bool {
        for r in 0..9 {
            if row <= r || r < row + 3 {
                continue;
            }
            if !Self::eliminate_multi(b, r, col, uc) {
                return false;
            }
        }
        true
    }
    fn triplex_ripple_row_block(b: &mut Board, row: usize, col: usize, uc: Candidates) -> bool {
        let row_start = row / 3 * 3;
        for r in row_start..row_start + 3 {
            for c in col..col + 3 {
                if r == row {
                    continue;
                }

                if !Self::eliminate_multi(b, r, c, uc) {
                    return false;
                }
            }
        }
        true
    }
    fn triplex_ripple_col_block(b: &mut Board, row: usize, col: usize, uc: Candidates) -> bool {
        let col_start = col / 3 * 3;
        for r in row..row + 3 {
            for c in col_start..col_start + 3 {
                if c == col {
                    continue;
                }

                if !Self::eliminate_multi(b, r, c, uc) {
                    return false;
                }
            }
        }
        true
    }
    fn eliminate_multi(b: &mut Board, row: usize, col: usize, rc: Candidates) -> bool {
        let mut ca = b.cell(row, col);
        ca.substract(&rc);
        if !ca.is_valid() {
            return false;
        }

        if ca == b.cell(row, col) {
            return true;
        } else {
            b.set_cell(row, col, ca);
        }

        if ca.is_done() && !Self::ripple(b, row, col) {
            return false;
        }
        true
    }
}
impl fmt::Display for Ripple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.g.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let s = "
		123456789
		.........
		987654321
		123456789
		.........
		987654321
		123456789
		.........
		987654321
		";
        assert_eq!(None, Ripple::solve(&s));

        let b = Ripple::new(&s[0..77]);
        assert_eq!(None, b);

        let solved = "
		123456789
		457389162
		869271453
		372594618
		581762394
		694813527
		715948236
		248635971
		936127845
		";
        let b = Ripple::new(&solved);
        assert_eq!(true, b.is_some());
        let mut b = b.unwrap();
        assert_eq!(true, b.do_solve());
        // assert_eq!("debug", b.to_string());
    }

    #[test]
    fn no_try() {
        let s = "
		..28.691.
		8.1......
		3....1.25
		6.9.1...4
		...659...
		1...2.7.9
		43.1....2
		......1.7
		.179.45..
		";
        let ans = Ripple::solve(&s);
        assert_eq!(true, ans.is_some());
        let ans = ans.unwrap();
        assert_eq!(81, ans.len());
        assert_eq!(b'2', ans.as_bytes()[2]);
        assert_eq!(b'5', ans.as_bytes()[0]);
        // assert_eq!("123", b.to_string());

        let s = "
		.58..64..
		........6
		7.21..3.9
		1..3.78..
		....2....
		..58.4..1
		9.1..52.7
		8........
		..76..14.
		";
        let mut b = Ripple::new(&s).unwrap();
        assert_eq!(true, b.do_solve());
        // assert_eq!("debug", b.to_string());
    }

    #[test]
    fn medium() {
        // hard?
        let s = "
		....7.19.
		.........
		4....2.87
		63..549..
		..17.64..
		..481..65
		82.9....3
		.........
		.16.4....
		";
        let mut b = Ripple::new(&s).unwrap();
        assert_eq!(true, b.do_solve());
        // assert_eq!("debug", b.to_string());
    }

    #[test]
    fn arto_inkala_2010() {
        let s = "..53.....8......2..7..1.5..4....53...1..7...6..32...8..6.5....9..4....3......97..";
        let mut b = Ripple::new(&s).unwrap();
        assert_eq!(true, b.do_solve());
    }

    #[test]
    fn huge_search() {
        let s = ".....6....59.....82....8....45........3........6..3.54...325..6..................";
        let mut b = Ripple::new(&s).unwrap();
        assert_eq!(true, b.do_solve());
    }
    #[test]
    fn hard() {
        // hard?
        let s = "
		8........
		..36.....
		.7..9.2..
		.5...7...
		....457..
		...1...3.
		..1....68
		..85...1.
		.9....4..
		";
        let mut b = Ripple::new(&s).unwrap();
        assert_eq!(true, b.do_solve());
    }

    #[test]
    fn hard_no_ans() {
        let s = "
		8........
		..36.....
		.7..9.2..
		.5...7...
		....457..
		...1...3.
		..12...68
		..85...1.
		.9....4..";
        let mut b = Ripple::new(&s).unwrap();
        assert_eq!(false, b.do_solve());
    }
}
