use std::env;
use std::fs;
use core::cmp;
use itertools::Itertools;
use euclid::*;
use std::fmt::Write;

pub type Vec2 = Vector2D<i32, UnknownUnit>;

struct Line {
	from: Vec2,
	dir: Vec2,
	dist: i32,
}

fn _by_manhattan(lines: Vec<Vec<Line>>) {

	let crosses = lines[0]
		.iter()
		.cartesian_product(lines[1].iter())
		.filter_map(|two_lines| {
			let line_a = two_lines.0; // p + t * r
			let line_b = two_lines.1; // q + u * s
			let rxs = line_a.dir.cross(line_b.dir);
			if rxs == 0 {
				None
			} else {
				let qp = line_b.from - line_a.from;
				let t = qp.cross(line_b.dir) as f32 / rxs as f32;
				let u = qp.cross(line_a.dir) as f32 / rxs as f32;
				let range = 0.0..1.0;
				if range.contains(&t) && range.contains(&u) {
					Some((line_a.from.to_f32() + line_a.dir.to_f32() * t).to_i32())
				} else {
					None
				}
			}
		});

	let crosses_str = crosses.clone()
		.fold(String::new(),|mut s, v| {write!(s, "({}, {}); ", v.x, v.y).ok(); s});

	println!("crosses: {}", crosses_str);

	crosses
		.map(|p| p.x.abs() + p.y.abs())
		.skip(1).min().unwrap();
}

fn main() {

	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	println!("In file {}", filename);

	let contents = fs::read_to_string(filename)
		.expect("Something went wrong reading the file");

	let lines: Vec<Vec<Line>> = contents
		.lines()
		.map(|line| {
			let mut origin = Vec2::new(0, 0);
			let mut steps_sum = 0;

			line.split(',')
				.map(|instr: &str| {
					let dir = instr.chars().next().unwrap();
					let dist = (&instr[1..]).parse::<i32>().unwrap();
					let vec = match dir {
						'R' => Vec2::new(dist, 0),
						'D' => Vec2::new(0, -dist),
						'U' => Vec2::new(0, dist),
						'L' => Vec2::new(-dist, 0),
						_ => panic!("unexpected input")
					};
					let from = origin;
					origin = origin + vec;
					steps_sum += dist;
					Line {
						from: from,
						dir: vec,
						dist: steps_sum - dist
					}
				})
				.collect()
		})
		.collect();

	let crosses = lines[0]
		.iter()
		.cartesian_product(lines[1].iter())
		.filter_map(|two_lines| {
			let line_a = two_lines.0; // p + t * r
			let line_b = two_lines.1; // q + u * s
			let rxs = line_a.dir.cross(line_b.dir);
			if rxs == 0 {
				None
			} else {
				let qp = line_b.from - line_a.from;
				let t = qp.cross(line_b.dir) as f32 / rxs as f32;
				let u = qp.cross(line_a.dir) as f32 / rxs as f32;
				let range = 0.0..1.0;
				if range.contains(&t) && range.contains(&u) {
					let dist_a = cmp::max(line_a.dir.x.abs(), line_a.dir.y.abs());
					let dist_b = cmp::max(line_b.dir.x.abs(), line_b.dir.y.abs());
					Some(Vec2::new(
						line_a.dist + (dist_a as f32 * t) as i32,
						line_b.dist + (dist_b as f32 * u) as i32))
				} else {
					None
				}
			}
		});

	let crosses_str = crosses.clone()
		.fold(String::new(),|mut s, v| {write!(s, "({}, {}); ", v.x, v.y).ok(); s});

	println!("crosses: {}", crosses_str);

	let res = crosses
		.map(|p| p.x.abs() + p.y.abs())
		.skip(1).min().unwrap();

	println!("res: {}", res);
}