use super::candidates::*;

use std::fmt;
use std::sync::Once;

static INIT: Once = Once::new();
static mut PEERS: [[[(u8, u8); 20]; 9]; 9] = [[[(0, 0); 20]; 9]; 9];

type Board = [[Candidates; 9]; 9];
#[derive(Debug, Default, PartialEq, Eq, Clone)]
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
	fn must_init() {
		INIT.call_once(|| unsafe {
			for row in 0..9 {
				for col in 0..9 {
					let mut k = 0;
					for i in 0..9 {
						// same row
						if i != col {
							PEERS[row][col][k] = (row as u8, i as u8);
							k += 1;
						}
						// same col
						if i != row {
							PEERS[row][col][k] = (i as u8, col as u8);
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
	fn peers_of(row: usize, col: usize) -> &'static [(u8, u8); 20] {
		unsafe { &PEERS[row][col] }
	}
	pub fn new(s: &str) -> Option<Self> {
		Self::must_init();

		let mut b: Self = Default::default();
		let mut i = 0;
		let mut j = 0;
		for c in s.chars() {
			if '1' <= c && c <= '9' || c == '.' {
				b.g[i][j] = Candidates::from(c);
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

	pub fn solve(s: &str) -> Option<String> {
		let mut b = if let Some(b) = Self::new(s) {
			b
		} else {
			return None;
		};

		if b.do_solve() {
			let mut s = String::with_capacity(81);
			for i in 0..9 {
				for j in 0..9 {
					// CAUTION: `to_string().chars().nth(0).unwrap()` takes 3000 ns...
					// s.push(b.g[i][j].to_string().chars().nth(0).unwrap());
					s.push(b.g[i][j].lucky());
				}
			}
			Some(s)
		} else {
			None
		}
	}
	pub fn solve_arr(puzzle: &mut [[char; 9]; 9]) -> bool {
		let mut b: Self = Default::default();
		for i in 0..9 {
			for j in 0..9 {
				b.g[i][j] = Candidates::from(puzzle[i][j]);
			}
		}

		if b.do_solve() {
			for i in 0..9 {
				for j in 0..9 {
					// CAUTION: `to_string().chars().nth(0).unwrap()` takes 3000 ns...
					// puzzle[i][j] = b.g[i][j].to_string().chars().nth(0).unwrap();
					puzzle[i][j] = b.g[i][j].lucky();
				}
			}
			true
		} else {
			false
		}
	}
	pub fn do_solve(&mut self) -> bool {
		// init
		for i in 0..9 {
			for j in 0..9 {
				if self.g[i][j].is_done() {
					if !Self::ripple(&mut self.g, i, j) {
						return false;
					}
				}
			}
		}
		// println!("init done:\n{}", self);

		if self.backtrack(&mut self.g.clone()) {
			// println!(
			// 	"solved:#try={},#triplex={}\n{}",
			// 	self.n_try, self.n_triplex, self
			// );
			true
		} else {
			// println!(
			// 	"unsovled:#try={},#triplex={}\n{}",
			// 	self.n_try, self.n_triplex, self
			// );
			false
		}
	}

	fn ripple(g: &mut Board, i: usize, j: usize) -> bool {
		let cs = g[i][j];
		for &(row, col) in Self::peers_of(i, j) {
			let (row, col) = (row as usize, col as usize);
			if g[row][col] == cs {
				return false;
			}
			if g[row][col].is_done() {
				continue;
			}
			g[row][col].substract(&cs);
			if g[row][col].is_done() {
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
		let mut g = g0.clone();
		for c in ca.iter() {
			self.n_try += 1;
			// make a guess
			g[row][col] = c;
			if Self::ripple(&mut g, row, col) {
				if self.backtrack(&mut g) {
					return true;
				}
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
		for row in 0..9 {
			for col in 0..9 {
				let ca = g[row][col];
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
		(min_row, min_col, g[min_row][min_col], hint)
	}
	fn triplex(&mut self, b: &mut Board, hint: Hint) -> bool {
		for i in 0..9 {
			for g in (0..9).step_by(3) {
				// row
				if i >= hint.first_unsolved_row {
					let (ca1, ca2, ca3) = (b[i][g], b[i][g + 1], b[i][g + 2]);
					if !(ca1.is_done() || ca2.is_done() || ca3.is_done()) {
						if ca1.len() <= 3 && ca2.len() <= 3 && ca3.len() <= 3 {
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
				}

				// col
				if i >= hint.first_unsolved_col {
					let (ca1, ca2, ca3) = (b[g][i], b[g + 1][i], b[g + 2][i]);
					if !(ca1.is_done() || ca2.is_done() || ca3.is_done()) {
						if ca1.len() <= 3 && ca2.len() <= 3 && ca3.len() <= 3 {
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
		let mut ca = b[row][col];
		ca.substract(&rc);
		if !ca.is_valid() {
			return false;
		}

		if ca == b[row][col] {
			return true;
		} else {
			b[row][col] = ca;
		}

		if ca.is_done() && !Self::ripple(b, row, col) {
			return false;
		}
		true
	}
}

impl fmt::Display for Ripple {
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
		let g = &self.g;
		// max width of each col, for alignment
		let cols_width = (0..9)
			.map(|col| (0..9).map(|row| g[row][col].len()).max().unwrap())
			.collect::<Vec<_>>();
		// total width with spaces
		let total_width = cols_width.iter().sum::<usize>() + 9 + 5;
		for i in 0..9 {
			// block line
			if i % 3 == 0 {
				f.write_str(" +")?;
				for _ in 0..total_width {
					f.write_str("-")?;
				}
				f.write_str("+\n")?;
			}
			for j in 0..9 {
				// block bound
				if j % 3 == 0 {
					f.write_str(" |")?;
				}
				// max width of this col
				let w = cols_width[j] + 1;
				// spaces required for this cell, if len==0, will show as 'X'
				let spaces = w - std::cmp::max(1, g[i][j].len());
				// leading spaces, put one more as leading
				for _ in 0..((spaces + 1) / 2) {
					f.write_str(" ")?
				}
				// candidates
				f.write_str(&g[i][j].to_string())?;
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
