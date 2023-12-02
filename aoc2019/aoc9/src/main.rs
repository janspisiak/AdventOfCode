// use std::env;
use std::{fs, collections::VecDeque, fmt::Write};
// use itertools::Itertools;

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq)]
enum OpCode {
    Add = 1,
    Mul = 2,
    Read = 3,
	Write = 4,
	JumpIfTrue = 5,
	JumpIfFalse = 6,
	CmpLess = 7,
	CmpEquals = 8,
	SpAdd = 9,
    Halt = 99,
}

#[derive(Debug)]
enum OpParamMode {
	Position,
	Immediate,
	Relative,
}

#[derive(Debug, Clone)]
enum OpParamType {
	Read,
	Write,
}

struct Operation {
	code: OpCode,
	param_types: Vec<OpParamType>,
	param_modes: Vec<OpParamMode>,
	params: Vec<i64>,
}

fn parse_opcode(ins: &str, op: &mut Operation) {
	type PT = OpParamType;
	let mut instr_str = ins.bytes().rev();
	let op_code_last = instr_str.next().unwrap();
	let code_slice = [instr_str.next().unwrap(), op_code_last];

	op.code = match &code_slice {
		b"01" => OpCode::Add,
		b"02" => OpCode::Mul,
		b"03" => OpCode::Read,
		b"04" => OpCode::Write,
		b"05" => OpCode::JumpIfTrue,
		b"06" => OpCode::JumpIfFalse,
		b"07" => OpCode::CmpLess,
		b"08" => OpCode::CmpEquals,
		b"09" => OpCode::SpAdd,
		b"99" | _ => OpCode::Halt
	};
	let types: &[OpParamType] = match op.code {
		OpCode::Add => &[PT::Read, PT::Read, PT::Write],
		OpCode::Mul => &[PT::Read, PT::Read, PT::Write],
		OpCode::Read => &[PT::Write],
		OpCode::Write => &[PT::Read],
		OpCode::JumpIfTrue => &[PT::Read, PT::Read],
		OpCode::JumpIfFalse => &[PT::Read, PT::Read],
		OpCode::CmpLess => &[PT::Read, PT::Read, PT::Write],
		OpCode::CmpEquals => &[PT::Read, PT::Read, PT::Write],
		OpCode::SpAdd => &[PT::Read],
		OpCode::Halt => &[],
	};
	op.param_types.clear();
	op.param_types.extend_from_slice(types);

	op.param_modes.clear();
	op.param_modes.extend(instr_str.take(types.len()).map(|c| {
		match c {
			b'0' => OpParamMode::Position,
			b'1' => OpParamMode::Immediate,
			b'2' => OpParamMode::Relative,
			_ => panic!("Unknown param mode \"{}\"", c as char),
		}
	}))
}

fn parse_params(cmp: &Computer, op: &mut Operation) {
	op.params.clear();
	op.params.extend(
		op.param_types.iter()
		.zip(op.param_modes.iter())
		.enumerate()
		.map(|(i, (ptype, mode))| {
			let val = cmp.mem[cmp.eip + 1 + i];
			match ptype {
				OpParamType::Write => match mode {
					OpParamMode::Position => val,
					OpParamMode::Immediate => val,
					OpParamMode::Relative => cmp.esp.checked_add_signed(val as isize).unwrap() as i64,
				},
				OpParamType::Read => match mode {
					OpParamMode::Position => *cmp.mem.get(val as usize).unwrap(),
					OpParamMode::Immediate => val,
					OpParamMode::Relative => cmp.mem[cmp.esp.checked_add_signed(val as isize).unwrap()],
				}
			}
		}))
}

#[derive(Clone)]
struct Computer {
	eip: usize,
    esp: usize,
	mem: Vec<i64>,
	ins: VecDeque<i64>,
	outs: VecDeque<i64>,
}

#[repr(i64)]
#[derive(Debug)]
enum ComputerState {
	Interrupt,
	Halted,
}

fn prog_run(cmp: &mut Computer) -> Option<ComputerState> {
	let mut op = Operation {
		code: OpCode::Halt,
		param_types: vec![],
		param_modes: vec![],
		params: vec![],
	};
	let mut op_str: String = String::new();

	while let Some(eix) = cmp.mem.get(cmp.eip).cloned() {
		op_str.clear();
		write!(op_str, "{:0>5}", eix).unwrap();
		parse_opcode(&op_str, &mut op);
		parse_params(cmp, &mut op);
		// let raw_slice = &cmp.mem[cmp.eip + 1..cmp.eip + op.params.len() + 1];
		// println!("[{}] {:#?}", cmp.eip, op.code);
		// println!("  esp: {}", cmp.esp);
		// println!("  ptypes: {:?}", op.param_types);
		// println!("  pmodes: {:?}", op.param_modes);
		// println!("  rparam: {:?}", raw_slice);
		// println!("  params: {:?}", op.params);

		match op.code {
			OpCode::Add | OpCode::Mul => {
				match op.params[..] {
					[eax, ebx, edi] => {
						let res = if op.code == OpCode::Add { eax + ebx } else { eax * ebx };
						cmp.mem[edi as usize] = res;
					}
					_ => return None,
				}
				cmp.eip += 4;
			}
			OpCode::Read => {
				match op.params[..] {
					[edi] => {
						let eax = cmp.ins.pop_front().unwrap();
						cmp.mem[edi as usize] = eax;
					}
					_ => return None,
				}
				cmp.eip += 2;
			}
			OpCode::Write => {
				match op.params[..] {
					[eax] => {
						cmp.outs.push_back(eax);
					}
					_ => return None,
				}
				cmp.eip += 2;
				return Some(ComputerState::Interrupt)
			}
			OpCode::JumpIfTrue | OpCode::JumpIfFalse => {
				match op.params[..] {
					[eax, ebx] => {
						// println!("[{}] {}: {} > 0 => [{}]", cmp.eip, eix, eax, ebx);
						let flag = match op.code {
							OpCode::JumpIfTrue => eax > 0,
							OpCode::JumpIfFalse => eax == 0,
							_ => false,
						};
						if flag {
							cmp.eip = ebx as usize;
						} else {
							cmp.eip += 3;
						}
					}
					_ => return None,
				}
			}
			OpCode::CmpLess => {
				match op.params[..] {
					[eax, ebx, edi] =>
						cmp.mem[edi as usize] = if eax < ebx { 1 } else { 0 },
					_ => return None,
				}
				cmp.eip += 4;
			}
			OpCode::CmpEquals => {
				match op.params[..] {
					[eax, ebx, edi] =>
						cmp.mem[edi as usize] = if eax == ebx { 1 } else { 0 },
					_ => return None,
				}
				cmp.eip += 4;
			}
			OpCode::SpAdd => {
				match op.params[..] {
					[eax] =>
						cmp.esp = cmp.esp.checked_add_signed(eax as isize).unwrap(),
					_ => return None,
				}
				cmp.eip += 2;
			}
			OpCode::Halt => {
				return Some(ComputerState::Halted)
			}
		}
	}
	Some(ComputerState::Halted)
}

fn main() {
	// let args: Vec<String> = env::args().collect();
	let prog_path = "aoc9/prog.txt";
	// println!("Prog at {}, input at {}", prog_path, input_path);

	let prog_str = fs::read_to_string(prog_path)
		.expect("Something went wrong reading the file");

	let orig_prog: Vec<i64> = prog_str
		.replace("\n", "")
		.split(',')
		.map(|line| line.parse::<i64>().unwrap())
		.collect();

	let mut comp_mem = orig_prog.clone();
	comp_mem.resize(8000, 0);
	let mut comp = Computer {
		eip: 0,
		esp: 0,
		mem: comp_mem,
		ins: VecDeque::from([2]),
		outs: VecDeque::new(),
	};
	let mut state = prog_run(&mut comp);
	while let Some(ComputerState::Interrupt) = state {
		state = prog_run(&mut comp);
	}

	println!("Result: {:?}", comp.outs);
}