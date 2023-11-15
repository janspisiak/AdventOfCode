use std::{collections::VecDeque, fs};

use crate::int::{Computer, ComputerState};
mod int;

enum SpringInstr {
    And(char, char),
    Or(char, char),
    Not(char, char),
    Walk,
    Run,
}

impl SpringInstr {
    fn to_ascii(&self) -> String {
        match self {
            Self::And(a, b) => format!("AND {} {}", a, b),
            Self::Or(a, b) => format!("OR {} {}", a, b),
            Self::Not(a, b) => format!("NOT {} {}", a, b),
            Self::Walk => "WALK".to_owned(),
            Self::Run => "RUN".to_owned(),
        }
    }
}

fn part_one(orig_prog: &Vec<i64>) {
    let mut comp = Computer {
        eip: 0,
        esp: 0,
        mem: orig_prog.clone(),
        ins: VecDeque::from([]),
        outs: VecDeque::new(),
    };

    // process until new line
    while let Some(_) = comp.run() {
        if let Some(out) = comp.outs.pop_front() {
            print!("{}", char::from(out as u8));
            if out == b'\n'.into() {
                break;
            }
        }
    }

    // A: any hole in first 3
    // B: last is path
    use SpringInstr::*;
    let _spring_prog_one = [
        Or('A', 'T'),
        And('B', 'T'),
        And('C', 'T'),
        // T is false if A
        Not('T', 'J'),
        // J is true if A
        And('D', 'J'),
        // J is true if B
        Walk,
    ];
    let spring_prog_two = [
        // same as first version
        Or('A', 'T'),
        And('B', 'T'),
        And('C', 'T'),
        Not('T', 'J'),
        And('D', 'J'),
        // But also check that either the after jump
        // or next jump is available path
        Or('E', 'T'),
        Or('H', 'T'),
        And('T', 'J'),
        Run,
    ];
    comp.ins.extend(spring_prog_two
        .map(|i| {
            let mut s = i.to_ascii();
            s.push_str("\n");
            s
        })
        .join("")
        .bytes()
        .map(|b| b as i64));
    let mut state = ComputerState::Interrupt;
    while state != ComputerState::Halted
    {
        state = comp.run().unwrap();
        if let Some(out) = comp.outs.pop_front() {
            if out < u8::MAX as i64 {
                print!("{}", char::from(out as u8));
            } else {
                println!("Success {}", out);
            }
        } else {
            // println!("No output");
            break;
        }
    }
}

fn main() {
    let prog_path = "aoc21/prog.txt";
    let prog_str = fs::read_to_string(prog_path).expect("Something went wrong reading the file");

    let mut orig_prog: Vec<i64> = prog_str
        .replace("\n", "")
        .split(',')
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    orig_prog.resize(8000, 0);

    part_one(&orig_prog);
}
