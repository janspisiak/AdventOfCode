use std::env;
use std::fs;
use std::iter::{from_fn, Iterator};

fn fuel_calc(from: i64) -> impl Iterator<Item = i64> {
	let mut current = from;
	
	from_fn(move || {
		current = (current / 3) - 2;
		if current > 0 { Some(current) } else { None }
	})
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	println!("In file {}", filename);

	let contents = fs::read_to_string(filename)
		.expect("Something went wrong reading the file");

	let sum: i64 = contents
		.lines()
		.map(|line| line.parse::<i64>())
		.inspect(|num| {
			if let Err(ref e) = *num {
				println!("Parsing error: {}", e);
			}
		})
		.filter_map(Result::ok)
		.flat_map(|x| fuel_calc(x))
		.sum();

	println!("Result: {}", sum);
}