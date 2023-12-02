use std::env;
use std::fs;

fn prog_run(mut prog: Vec<i64>) -> Vec<i64> {
	let mut eip: usize = 0;
	while let Some(eix) = prog.get(eip).cloned() {
		match eix {	
			1 => {
				// address of a, b, d
				let (peax, pebx, edi) = (prog[eip + 1], prog[eip + 2], prog[eip + 3]);
				let result: i64 = prog[peax as usize] + prog[pebx as usize];
				//println!("[{}] {}: [{}] {} + [{}] {} => [{}] {}", eip, eix, peax, prog[peax as usize], pebx, prog[pebx as usize], edi, result);
				prog[edi as usize] = result;
				eip += 4
			}
			2 => {
				// address of a, b, d
				let (peax, pebx, edi) = (prog[eip + 1], prog[eip + 2], prog[eip + 3]);
				let result: i64 = prog[peax as usize] * prog[pebx as usize];
				//println!("[{}] {}: [{}] {} * [{}] {} => [{}] {}", eip, eix, peax, prog[peax as usize], pebx, prog[pebx as usize], edi, result);
				prog[edi as usize] = result;
				eip += 4
			}
			99 | _ => {
				break
			}
		}
	}
	prog
}

fn main() {
	use std::fmt::Write;

	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	println!("In file {}", filename);

	let contents = fs::read_to_string(filename)
		.expect("Something went wrong reading the file");

	let orig_prog: Vec<i64> = contents
		.split(',')
		.map(|line| line.parse::<i64>().unwrap())
		.collect();

	let result: Option<Vec<i64>> = {
		let mut result: Option<Vec<i64>> = None;
		for noun in 0..99 {
			for verb in 0..99 {
				let mut new_prog = orig_prog.clone();
				new_prog[1] = noun;
				new_prog[2] = verb;
				let res: Vec<i64> = prog_run(new_prog);
				if res[0] == 19690720 {
					result = Some(res);
					break
				}
			}
		}
		result
	};

	let res_str = result.unwrap()
		.iter()
		.enumerate()
		.fold(String::new(),|mut s, (i, v)| {write!(s, "[{}:{}] ", i, v).ok(); s});

	println!("Result: {}", res_str);
}