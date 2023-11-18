use std::{collections::VecDeque, fmt::Write};


#[allow(dead_code)]
#[derive(Debug, PartialEq)]
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
        b"99" | _ => OpCode::Halt,
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
    op.param_modes
        .extend(instr_str.take(types.len()).map(|c| match c {
            b'0' => OpParamMode::Position,
            b'1' => OpParamMode::Immediate,
            b'2' => OpParamMode::Relative,
            _ => panic!("Unknown param mode \"{}\"", c as char),
        }))
}

fn parse_params(cmp: &Computer, op: &mut Operation) {
    op.params.clear();
    op.params.extend(
        op.param_types
            .iter()
            .zip(op.param_modes.iter())
            .enumerate()
            .map(|(i, (ptype, mode))| {
                let val = cmp.mem[cmp.eip + 1 + i];
                match ptype {
                    OpParamType::Write => match mode {
                        OpParamMode::Position => val,
                        OpParamMode::Immediate => val,
                        OpParamMode::Relative => {
                            cmp.esp.checked_add_signed(val as isize).unwrap() as i64
                        }
                    },
                    OpParamType::Read => match mode {
                        OpParamMode::Position => *cmp.mem.get(val as usize).unwrap(),
                        OpParamMode::Immediate => val,
                        OpParamMode::Relative => {
                            cmp.mem[cmp.esp.checked_add_signed(val as isize).unwrap()]
                        }
                    },
                }
            }),
    )
}

#[derive(Clone)]
pub struct Computer {
    pub eip: usize,
    pub esp: usize,
    pub mem: Vec<i64>,
    pub ins: VecDeque<i64>,
    pub outs: VecDeque<i64>,
}

#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComputerState {
    ReadInt,
    WriteInt,
    Halted,
}

impl Computer {
    pub fn run(&mut self) -> Option<ComputerState> {
        let mut op = Operation {
            code: OpCode::Halt,
            param_types: vec![],
            param_modes: vec![],
            params: vec![],
        };
        let mut op_str: String = String::new();

        while let Some(eix) = self.mem.get(self.eip).cloned() {
            op_str.clear();
            write!(op_str, "{:0>5}", eix).unwrap();
            parse_opcode(&op_str, &mut op);
            parse_params(self, &mut op);
            // let raw_slice = &cmp.mem[self.eip + 1..self.eip + op.params.len() + 1];
            // println!("[{}] {:#?}", self.eip, op.code);
            // println!("  esp: {}", self.esp);
            // println!("  ptypes: {:?}", op.param_types);
            // println!("  pmodes: {:?}", op.param_modes);
            // println!("  rparam: {:?}", raw_slice);
            // println!("  params: {:?}", op.params);

            match op.code {
                OpCode::Add | OpCode::Mul => {
                    match op.params[..] {
                        [eax, ebx, edi] => {
                            let res = if op.code == OpCode::Add {
                                eax + ebx
                            } else {
                                eax * ebx
                            };
                            self.mem[edi as usize] = res;
                        }
                        _ => return None,
                    }
                    self.eip += 4;
                }
                OpCode::Read => {
                    if let Some(eax) = self.ins.pop_front() {
                        match op.params[..] {
                            [edi] => {
                                self.mem[edi as usize] = eax;
                            }
                            _ => return None,
                        }
                        self.eip += 2;
                    } else {
                        return Some(ComputerState::ReadInt);
                    }
                }
                OpCode::Write => {
                    match op.params[..] {
                        [eax] => {
                            self.outs.push_back(eax);
                        }
                        _ => return None,
                    }
                    self.eip += 2;
                    return Some(ComputerState::WriteInt);
                }
                OpCode::JumpIfTrue | OpCode::JumpIfFalse => {
                    match op.params[..] {
                        [eax, ebx] => {
                            // println!("[{}] {}: {} > 0 => [{}]", self.eip, eix, eax, ebx);
                            let flag = match op.code {
                                OpCode::JumpIfTrue => eax > 0,
                                OpCode::JumpIfFalse => eax == 0,
                                _ => false,
                            };
                            if flag {
                                self.eip = ebx as usize;
                            } else {
                                self.eip += 3;
                            }
                        }
                        _ => return None,
                    }
                }
                OpCode::CmpLess => {
                    match op.params[..] {
                        [eax, ebx, edi] => self.mem[edi as usize] = if eax < ebx { 1 } else { 0 },
                        _ => return None,
                    }
                    self.eip += 4;
                }
                OpCode::CmpEquals => {
                    match op.params[..] {
                        [eax, ebx, edi] => self.mem[edi as usize] = if eax == ebx { 1 } else { 0 },
                        _ => return None,
                    }
                    self.eip += 4;
                }
                OpCode::SpAdd => {
                    match op.params[..] {
                        [eax] => self.esp = self.esp.checked_add_signed(eax as isize).unwrap(),
                        _ => return None,
                    }
                    self.eip += 2;
                }
                OpCode::Halt => return Some(ComputerState::Halted),
            }
        }
        Some(ComputerState::Halted)
    }
}
