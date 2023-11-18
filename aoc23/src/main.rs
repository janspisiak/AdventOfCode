use std::{collections::VecDeque, fs};

use itertools::Itertools;

use crate::int::{Computer, ComputerState};
mod int;

fn part_one(orig_prog: &Vec<i64>) {
    let comps_len = 50usize;
    let mut comps = Vec::from_iter((0..comps_len).map(|i| Computer {
        eip: 0,
        esp: 0,
        mem: orig_prog.clone(),
        ins: VecDeque::from([i as i64]),
        outs: VecDeque::new(),
    }));

    let mut states = vec![ComputerState::Halted; comps_len];
    let mut packets = vec![];
    let mut nat = Some((0, 0, 0));
    let mut frame = 0;
    loop {
        frame += 1;
        for (i, c) in comps.iter_mut().enumerate() {
            states[i] = c.run().unwrap();
            if c.outs.len() >= 3 {
                // we have packet
                let (addr, x, y) = c.outs.drain(..3).collect_tuple().unwrap();
                if addr > 0 && (addr as usize) < comps_len {
                    packets.push((addr, x, y));
                    println!("[{frame}] Packet {i} -> {addr} {x} {y}");
                } else {
                    nat = Some((0i64, x, y));
                    println!("[{frame}] Nat Packet {addr} {x} {y}");
                }
            }
        }
        packets.drain(..).for_each(|p| {
            let (addr, x, y) = p;
            comps[addr as usize].ins.extend([x, y].iter());
        });
        let all_idle = comps.iter()
            .enumerate()
            .all(|(i, c)| c.ins.is_empty()
                && c.outs.is_empty()
                && states[i] != ComputerState::WriteInt);
        if all_idle {
            if let Some(p) = nat {
                let (count, x, y) = p;
                if count > 0 {
                    println!("nat retrigger {y}");
                    break;
                }
                comps[0].ins.extend([x, y].iter());
                nat = Some((count + 1, x, y));
            } else {
                println!("[{frame}] No NAT and all_idle");
            }
        }
        // fill empty queues
        for c in comps.iter_mut() {
            if c.ins.is_empty() {
                c.ins.push_back(-1);
            }
        }
    }
}

fn main() {
    let prog_path = "aoc23/prog.txt";
    let prog_str = fs::read_to_string(prog_path).expect("Something went wrong reading the file");

    let mut orig_prog: Vec<i64> = prog_str
        .replace("\n", "")
        .split(',')
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    orig_prog.resize(8000, 0);

    part_one(&orig_prog);
}
