#![feature(test)] // #[bench] is still experimental

extern crate test; // Even in '18 this is needed ... for reasons.

use test::{black_box, Bencher}; // `black_box` prevents `f` from being optimized away.

use libsudoku::candidates::*;
use libsudoku::ripple::*;

const EASY: &str = "
9165384..
.2.......
.87....31
6.3.1..8.
7..863..5
.5..9.6..
13....25.
.......74
4752.63..
";
#[bench]
fn easy(b: &mut Bencher) {
	b.iter(|| black_box(Ripple::solve(EASY)));
}

fn to_arr(s: &str) -> [[char; 9]; 9] {
	let mut arr = [['.'; 9]; 9];
	let mut i = 0;
	for c in s.chars() {
		if '1' <= c && c <= '9' || c == '.' {
			let (row, col) = (i / 9, i % 9);
			arr[row][col] = c;
			i += 1;
		}
	}
	arr
}
#[bench]
fn s_to_arr(b: &mut Bencher) {
	b.iter(|| black_box(to_arr(EASY)));
}
#[bench]
fn result_to_str(b: &mut Bencher) {
	let mut arr = to_arr(EASY);
	Ripple::solve_arr(&mut arr);
	let mut board = [[Candidates::default(); 9]; 9];
	for i in 0..9 {
		for j in 0..9 {
			board[i][j] = Candidates::from(arr[i][j]);
		}
	}
	b.iter(|| {
		let mut s = String::with_capacity(81);
		for i in 0..9 {
			for j in 0..9 {
				// s.push(board[i][j].to_string().chars().nth(0).unwrap());
				s.push(board[i][j].lucky());
			}
		}
		black_box(s)
	});
}
#[bench]
fn result_to_arr(b: &mut Bencher) {
	let mut arr = to_arr(EASY);
	Ripple::solve_arr(&mut arr);
	let mut board = [[Candidates::default(); 9]; 9];
	for i in 0..9 {
		for j in 0..9 {
			board[i][j] = Candidates::from(arr[i][j]);
		}
	}
	b.iter(|| {
		let mut ans = [['.'; 9]; 9];
		for i in 0..9 {
			for j in 0..9 {
				// ans[i][j] = board[i][j].to_string().chars().nth(0).unwrap();
				ans[i][j] = board[i][j].lucky();
			}
		}
		black_box(ans)
	});
}
#[bench]
fn easy_arr(b: &mut Bencher) {
	let arr = to_arr(EASY);
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(Ripple::solve_arr(&mut arr))
	});
}

const EASY_3_ANS: &str = "
9165384..
.2.......
.87....31
6.3.1..8.
7..863..5
.5..9.6..
.3....25.
.......74
4752.63..
";

#[bench]
fn easy_3_ans(b: &mut Bencher) {
	b.iter(|| black_box(Ripple::solve(EASY_3_ANS)));
}

const MEDIUM: &str = "
4..853.69
.........
.95....2.
7....5...
6...4.21.
.1...8..4
5......42
.4..9....
3.1..6...";
#[bench]
fn medium(b: &mut Bencher) {
	b.iter(|| black_box(Ripple::solve(MEDIUM)));
}

const HARD: &str = "
8........
..36.....
.7..9.2..
.5...7...
....457..
...1...3.
..1....68
..85...1.
.9....4..";

#[bench]
fn hard(b: &mut Bencher) {
	b.iter(|| black_box(Ripple::solve(HARD)));
}

const HARD_NO_ANS: &str = "
8........
..36.....
.7..9.2..
.5...7...
....457..
...1...3.
..12...68
..85...1.
.9....4..";

#[bench]
fn hard_no_ans(b: &mut Bencher) {
	b.iter(|| black_box(Ripple::solve(HARD_NO_ANS)));
}

const HUGE_SEARCH: &str = "
	.....6...
	.59.....8
	2....8...
	.45......
	..3......
	..6..3.54
	...325..6
	.........
	.........";
#[bench]
// #[ignore] // ~600ms
fn huge_search(b: &mut Bencher) {
	b.iter(|| black_box(Ripple::solve(HUGE_SEARCH)));
}
