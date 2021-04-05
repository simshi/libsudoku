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
				.long("puzzle")
				.takes_value(true)
				.required(true)
				.help("The puzzle, only '1-9' and '.' are valid, others are ignored"),
		)
		.get_matches();

	let puzzle = args.value_of("PUZZLE").unwrap();

	if let Some(mut game) = Ripple::new(&puzzle) {
		if game.do_solve() {
			println!("answer:\n{}", game.to_string());
		} else {
			println!("no answer")
		}
	} else {
		println!("invalid input")
	}
}
