// use std::env;
use std::fs;

#[allow(dead_code)]
#[repr(i64)]
#[derive(Debug)]
enum OpCode {
    Add = 1,
    Mul = 2,
    Read = 3,
	Write = 4,
	JumpIfTrue = 5,
	JumpIfFalse = 6,
	CmpLess = 7,
	CmpEquals = 8,
    Halt = 99,
}

fn parse_param(mem: &[i64], mode: char, val: i64) -> i64 {
	match mode {
		'0' => mem[val as usize],
		'1' | _ => val,
	}
}

fn prog_run(mut prog: Vec<i64>, inputs: &mut Vec<i64>, outputs: &mut Vec<i64>) -> Vec<i64> {
	let mut eip: usize = 0;
	while let Some(eix) = prog.get(eip).cloned() {
		let op_str = format!("{:0>5}", eix);
		let fixed: Vec<_> = op_str.chars().collect();
        let eix_code: OpCode = match &op_str[3..5] {
			"01" => OpCode::Add,
			"02" => OpCode::Mul,
			"03" => OpCode::Read,
			"04" => OpCode::Write,
			"05" => OpCode::JumpIfTrue,
			"06" => OpCode::JumpIfFalse,
			"07" => OpCode::CmpLess,
			"08" => OpCode::CmpEquals,
			"99" | _ => OpCode::Halt
		};
		println!("[{}] {:#?}", eip, eix_code);
		match eix_code {
			OpCode::Add => {
				let eax = parse_param(&prog, fixed[2], prog[eip + 1]);
				let ebx = parse_param(&prog, fixed[1], prog[eip + 2]);
				let edi = prog[eip + 3];
				//println!("[{}] {}: [{}] {} + [{}] {} => [{}] {}", eip, eix, peax, prog[peax as usize], pebx, prog[pebx as usize], edi, result);
				prog[edi as usize] = eax + ebx;
				eip += 4;
			}
			OpCode::Mul => {
				let eax = parse_param(&prog, fixed[2], prog[eip + 1]);
				let ebx = parse_param(&prog, fixed[1], prog[eip + 2]);
				let edi = prog[eip + 3];
				//println!("[{}] {}: [{}] {} * [{}] {} => [{}] {}", eip, eix, peax, prog[peax as usize], pebx, prog[pebx as usize], edi, result);
				prog[edi as usize] = eax * ebx;
				eip += 4;

			}
			OpCode::Read => {
				let eax = inputs.pop().unwrap();
				let edi = prog[eip + 1];
				prog[edi as usize] = eax;
				eip += 2;
			}
			OpCode::Write => {
				let eax = parse_param(&prog, fixed[2], prog[eip + 1]);
				outputs.push(eax);
				eip += 2;
			}
			OpCode::JumpIfTrue => {
				let eax = parse_param(&prog, fixed[2], prog[eip + 1]);
				let ebx = parse_param(&prog, fixed[1], prog[eip + 2]);
				println!("[{}] {}: {} > 0 => [{}]", eip, eix, eax, ebx);
				if eax > 0 {
					eip = ebx as usize;
				} else {
					eip += 3;
				}
			}
			OpCode::JumpIfFalse => {
				let eax = parse_param(&prog, fixed[2], prog[eip + 1]);
				let ebx = parse_param(&prog, fixed[1], prog[eip + 2]);
				if eax == 0 {
					eip = ebx as usize;
				} else {
					eip += 3;
				}
			}
			OpCode::CmpLess => {
				let eax = parse_param(&prog, fixed[2], prog[eip + 1]);
				let ebx = parse_param(&prog, fixed[1], prog[eip + 2]);
				let edi = prog[eip + 3];
				prog[edi as usize] = if eax < ebx { 1 } else { 0 };
				eip += 4;
			}
			OpCode::CmpEquals => {
				let eax = parse_param(&prog, fixed[2], prog[eip + 1]);
				let ebx = parse_param(&prog, fixed[1], prog[eip + 2]);
				let edi = prog[eip + 3];
				prog[edi as usize] = if eax == ebx { 1 } else { 0 };
				eip += 4;
			}
			OpCode::Halt => {
				break
			}
		}
	}
	prog
}

fn main() {
	use std::fmt::Write;

	// let args: Vec<String> = env::args().collect();
	let prog_path = "aoc5/prog.txt";
	let input_path = "aoc5/input.txt";
	// println!("Prog at {}, input at {}", prog_path, input_path);

	let prog_str = fs::read_to_string(prog_path)
		.expect("Something went wrong reading the file");

	let input_str = fs::read_to_string(input_path)
		.expect("Something went wrong reading the file");

	let orig_prog: Vec<i64> = prog_str
		.replace('\n', "")
		.split(',')
		.map(|line| line.parse::<i64>().unwrap())
		.collect();

	let mut inputs: Vec<_> = input_str
		.split(',')
		.map(|l| l.parse::<i64>().unwrap())
		.collect();
	let mut outputs: Vec<i64> = Vec::new();

	let _prog_end = prog_run(orig_prog, &mut inputs, &mut outputs);

	let res_str = outputs
		.iter()
		.enumerate()
		.fold(String::new(),|mut s, (_i, v)| {write!(s, "{}, ", v).ok(); s});

	println!("Result: {}", res_str);
}