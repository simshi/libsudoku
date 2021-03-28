use clap::{App, AppSettings, Arg};

use shudu::ripple::*;

pub fn main() {
	let args = App::new("Shudu")
		.version("0.1.0")
		.author("Simon Shi <simonshi@gmail.com>")
		.about("Shudo(sudoku) solver in Rust")
		.setting(AppSettings::ArgRequiredElseHelp)
		.arg(
			Arg::with_name("PUZZLE")
				.short("p")
				.takes_value(true)
				.help("Puzzle as string, only '1-9' and '.' are valid, others are ignored"),
		)
		.get_matches();

	if let Some(puzzle) = args.value_of("PUZZLE") {
		if let Some(mut game) = Ripple::new(&puzzle) {
			// println!("solving:\n{}", game.to_string());
			if game.do_solve() {
				println!("answer:\n{}", game.to_string());
			} else {
				println!("no answer")
			}
		} else {
			println!("invalid input")
		}
	}
}
