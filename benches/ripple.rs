use criterion::{black_box, BatchSize, Criterion};
use criterion::{criterion_group, criterion_main};

use sudoku::candidates::*;
use sudoku::ripple::*;

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
fn str_to_arr(c: &mut Criterion) {
    c.bench_function("str_to_arr", |b| b.iter(|| black_box(to_arr(EASY))));
}

fn result_to_str(c: &mut Criterion) {
    fn setup() -> ([[Candidates; 9]; 9], String) {
        let mut arr = to_arr(EASY);
        Ripple::solve_arr(&mut arr);
        let mut board = [[Candidates::default(); 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                board[i][j] = Candidates::from(arr[i][j]);
            }
        }
        (board, String::with_capacity(81))
    }

    let mut group = c.benchmark_group("result_to_str");
    group.bench_function("chars", |b| {
        b.iter_batched(
            || setup(),
            |(board, s)| {
                let mut s = s.clone();
                for i in 0..9 {
                    for j in 0..9 {
                        s.push(board[i][j].to_string().chars().nth(0).unwrap());
                    }
                }
                black_box(s);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("lucky", |b| {
        b.iter_batched(
            || setup(),
            |(board, s)| {
                let mut s = s.clone();
                for i in 0..9 {
                    for j in 0..9 {
                        s.push(board[i][j].lucky());
                    }
                }
                black_box(s);
            },
            BatchSize::SmallInput,
        );
    });
}

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

fn ripple_solver(c: &mut Criterion) {
    let mut group = c.benchmark_group("ripple_solver");

    group.bench_function("easy", |b| {
        b.iter(|| black_box(Ripple::solve(EASY)));
    });

    group.bench_function("easy_arr", |b| {
        b.iter_batched(
            || to_arr(EASY),
            |arr| {
                let mut arr = arr.clone();
                black_box(Ripple::solve_arr(&mut arr));
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("easy_3_ans", |b| {
        b.iter(|| black_box(Ripple::solve(EASY_3_ANS)));
    });

    group.bench_function("medium", |b| {
        b.iter(|| black_box(Ripple::solve(MEDIUM)));
    });

    group.bench_function("hard", |b| {
        b.iter(|| black_box(Ripple::solve(HARD)));
    });

    group.bench_function("hard_no_ans", |b| {
        b.iter(|| black_box(Ripple::solve(HARD_NO_ANS)));
    });

    group.bench_function("huge_search", |b| {
        b.iter(|| black_box(Ripple::solve(HUGE_SEARCH)));
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = str_to_arr,result_to_str,ripple_solver
}
criterion_main!(benches);
