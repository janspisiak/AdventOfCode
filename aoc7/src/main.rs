// use std::env;
use std::{fs, collections::VecDeque};
use itertools::Itertools;

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

#[derive(Clone)]
struct Amp {
	mem: Vec<i64>,
	eip: usize,
	ins: VecDeque<i64>,
	outs: VecDeque<i64>,
}

fn prog_run(prog: &mut Vec<i64>, inputs: &mut VecDeque<i64>, outputs: &mut VecDeque<i64>, mut eip: usize) -> (bool, usize) {
	let mut halted = false;
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
		// println!("[{}] {:#?}", eip, eix_code);
		match eix_code {
			OpCode::Add => {
				let eax = parse_param(prog, fixed[2], prog[eip + 1]);
				let ebx = parse_param(prog, fixed[1], prog[eip + 2]);
				let edi = prog[eip + 3];
				//println!("[{}] {}: [{}] {} + [{}] {} => [{}] {}", eip, eix, peax, prog[peax as usize], pebx, prog[pebx as usize], edi, result);
				prog[edi as usize] = eax + ebx;
				eip += 4;
			}
			OpCode::Mul => {
				let eax = parse_param(prog, fixed[2], prog[eip + 1]);
				let ebx = parse_param(prog, fixed[1], prog[eip + 2]);
				let edi = prog[eip + 3];
				//println!("[{}] {}: [{}] {} * [{}] {} => [{}] {}", eip, eix, peax, prog[peax as usize], pebx, prog[pebx as usize], edi, result);
				prog[edi as usize] = eax * ebx;
				eip += 4;

			}
			OpCode::Read => {
				let eax = inputs.pop_front().unwrap();
				let edi = prog[eip + 1];
				prog[edi as usize] = eax;
				eip += 2;
			}
			OpCode::Write => {
				let eax = parse_param(prog, fixed[2], prog[eip + 1]);
				outputs.push_back(eax);
				eip += 2;
				break
			}
			OpCode::JumpIfTrue => {
				let eax = parse_param(prog, fixed[2], prog[eip + 1]);
				let ebx = parse_param(prog, fixed[1], prog[eip + 2]);
				println!("[{}] {}: {} > 0 => [{}]", eip, eix, eax, ebx);
				if eax > 0 {
					eip = ebx as usize;
				} else {
					eip += 3;
				}
			}
			OpCode::JumpIfFalse => {
				let eax = parse_param(prog, fixed[2], prog[eip + 1]);
				let ebx = parse_param(prog, fixed[1], prog[eip + 2]);
				if eax == 0 {
					eip = ebx as usize;
				} else {
					eip += 3;
				}
			}
			OpCode::CmpLess => {
				let eax = parse_param(prog, fixed[2], prog[eip + 1]);
				let ebx = parse_param(prog, fixed[1], prog[eip + 2]);
				let edi = prog[eip + 3];
				prog[edi as usize] = if eax < ebx { 1 } else { 0 };
				eip += 4;
			}
			OpCode::CmpEquals => {
				let eax = parse_param(prog, fixed[2], prog[eip + 1]);
				let ebx = parse_param(prog, fixed[1], prog[eip + 2]);
				let edi = prog[eip + 3];
				prog[edi as usize] = if eax == ebx { 1 } else { 0 };
				eip += 4;
			}
			OpCode::Halt => {
				halted = true;
				break
			}
		}
	}
	(halted, eip)
}

fn main() {
	// let args: Vec<String> = env::args().collect();
	let prog_path = "aoc7/prog.txt";
	// println!("Prog at {}, input at {}", prog_path, input_path);

	let prog_str = fs::read_to_string(prog_path)
		.expect("Something went wrong reading the file");

	let orig_prog: Vec<i64> = prog_str
		.replace('\n', "")
		.split(',')
		.map(|line| line.parse::<i64>().unwrap())
		.collect();

	let mut max_thrust = 0i64;
	for phases in (5..10).permutations(5) {
		let mut input = 0i64;
		let mut amps: Vec<Amp> = (0..5).map(|i| {
			Amp {
				mem: orig_prog.clone(),
				eip: 0,
				ins: VecDeque::from([phases[i]]),
				outs: VecDeque::new(),
			}
		}).collect();
		for amp_i in (0..5).cycle() {
			// TODO supply phase only on init
			let amp = &mut amps[amp_i];
			amp.ins.push_back(input);
			let (halted, new_eip) = prog_run(&mut amp.mem, &mut amp.ins, &mut amp.outs, amp.eip);
			amp.eip = new_eip;
			println!("Amp {} [{}, {}] {:#?} {}", amp_i, phases[amp_i], input, amp.outs, amp.eip);
			if let Some(inp) = amp.outs.get(0) {
				input = *inp;
			}
			amp.outs.clear();
			if halted { break }
		}
		max_thrust = std::cmp::max(max_thrust, input);
	}

	println!("Result: {:#?}", max_thrust);
}