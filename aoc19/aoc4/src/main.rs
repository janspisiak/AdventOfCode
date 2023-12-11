// use std::env;
use itertools::Itertools;

fn str_to_digits(s: &str) -> Vec<u32> {
	return s.chars().map(|c| c.to_digit(10).unwrap()).collect();
}

fn main() {
	// let args: Vec<String> = env::args().collect();
	let range_start: u32 = 136818;
	let range_end: u32 = 685979;

	let matches: Vec<u32> = (range_start..range_end)
		.filter(|n| {
			let dig_vec = str_to_digits(&n.to_string());
			let mut groups: Vec<Vec<u32>> = Vec::new();
			for (_, group)
			 in &dig_vec.to_vec().into_iter().group_by(|x| *x) {
				groups.push(group.collect());
			}
			dig_vec.windows(2).all(|p| p[0] <= p[1])
			 && dig_vec.windows(2).any(|p| p[0] == p[1])
			 && groups.into_iter().any(|g| g.len() == 2)
		})
		.collect();

	// println!("res: {:#?} {:#?}", start_digs, end_digs);
	println!("count: {}", matches.len());
	println!("some: {:#?}", &matches[..16]);
}