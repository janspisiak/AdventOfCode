use core::panic;
use std::{collections::VecDeque, fs, ops::Range};
use euclid::{Vector2D, UnknownUnit, vec2};
use itertools::Itertools;

use crate::int::{Computer, ComputerState};
mod int;

type Vec2i = Vector2D<i32, UnknownUnit>;

trait VectorRotate {
    fn rotate_left(&self) -> Self;
    fn rotate_right(&self) -> Self;
}

impl VectorRotate for Vec2i {
    fn rotate_left(&self) -> Vec2i {
        vec2(self.y, -self.x)
    }
    fn rotate_right(&self) -> Vec2i {
        vec2(-self.y, self.x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tiles {
    Path,
    Empty,
    Robot,
}

impl From<u8> for Tiles {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Tiles::Path,
            b'.' => Tiles::Empty,
            b'^' | b'v' | b'<' | b'>' | b'X' => Tiles::Robot,
            _ => panic!("Unknown tile")
        }
    }
}

impl Into<char> for Tiles {
    fn into(self) -> char {
        match self {
            Self::Empty => '.',
            Self::Path => '#',
            Self::Robot => 'R',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Command {
    Move(i32),
    Left,
    Right,
}

struct Map {
    tsize: Vec2i,
    tiles: Vec<Tiles>,
}

const ALL_DIRS: &[Vec2i] = &[
    vec2( 0, -1),
    vec2( 0,  1),
    vec2(-1,  0),
    vec2( 1,  0),
];

fn inside_size(size: Vec2i) -> impl Fn(&Vector2D<i32, UnknownUnit>) -> bool {
    move |pos: &Vec2i| {
        !pos.lower_than(vec2(0, 0)).any() &&
        !pos.greater_than(vec2(size.x - 1, size.y - 1)).any()
    }
}

impl Map {
    fn tile(&self, pos: Vec2i) -> &Tiles {
        &self.tiles[(self.tsize.x * pos.y + pos.x) as usize]
    }
    fn tile_mut(&mut self, pos: Vec2i) -> &mut Tiles {
        &mut self.tiles[(self.tsize.x * pos.y + pos.x) as usize]
    }
    fn shrink_tiles(&mut self) {
        self.tiles.truncate((self.tsize.x * self.tsize.y) as usize);
    }

    fn path_cross(&self) -> Vec<Vec2i> {
        self.tiles.iter()
            .enumerate()
            .filter_map(|(i, &t)| {
                if t == Tiles::Path {
                    let pos = vec2(i as i32 % self.tsize.x, i as i32 / self.tsize.x);
                    let is_cross = |&pos: &Vec2i| {
                        ALL_DIRS.iter()
                            .filter_map(move |&d| {
                                Some(pos + d).filter(inside_size(self.tsize))
                            })
                            .all(|p| *self.tile(p) == Tiles::Path)
                    };
                    Some(pos).filter(is_cross)
                } else {
                    None
                }
            })
            .collect()
    }

    fn _debug_tiles(&self) {
        let rows: Vec<_> = self.tiles.chunks(self.tsize.x as usize).into_iter()
            .map(|ch| -> Vec<char> {
                ch.iter().map(|&t| -> char { t.into() }).collect()
            })
            .collect();
        rows.iter().filter_map(|r| {
            let s = String::from_iter(r.iter());
            if s.trim().is_empty() {
                None
            } else {
                Some(s)
            }
        }).for_each(|s| println!("{}", s));
        println!("");
    }

}

fn main() {
    let prog_path = "aoc17/prog.txt";
    let prog_str = fs::read_to_string(prog_path).expect("Something went wrong reading the file");

    let orig_prog: Vec<i64> = prog_str
        .replace("\n", "")
        .split(',')
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut comp_mem = orig_prog.clone();
    comp_mem.resize(8000, 0);
    // enable robot movement
    comp_mem[0] = 2;
    let mut comp = Computer {
        eip: 0,
        esp: 0,
        mem: comp_mem,
        ins: VecDeque::from([]),
        outs: VecDeque::new(),
    };

    let mut map = Map {
        tsize: vec2(0, 0),
        tiles: vec![Tiles::Empty; 100 * 100],
    };

    // let mut robot_pos: Vec2i = start_pos;
    // let mut path_to: Vec<Dir> = vec![Dir::North];
    let mut map_pos: Vec2i = vec2(0, 0);
    let mut robot_pos: Vec2i = vec2(0, 0);
    let mut robot_dir: Vec2i = vec2(0, -1);
    let mut state = ComputerState::Interrupt;
    while ComputerState::Interrupt == state {
        state = comp.run().unwrap();
        if comp.outs.len() >= 1 {
            // let hit = Tiles::from(comp.outs.pop_front().unwrap());
            let out_ch = comp.outs.pop_front()
                .and_then(|v| u8::try_from(v).ok())
                .unwrap();

            // initial video feed of map
            if map.tsize.y == 0 {
                if out_ch == b'\n' {
                    // save map width
                    if map.tsize.x == 0 {
                        map.tsize.x = map_pos.x;
                    }
                    // save map height
                    if map_pos.x == 0 {
                        map.tsize.y = map_pos.y;
                        break;
                    }
                    map_pos.y += 1;
                    map_pos.x = 0;
                } else {
                    let tile = Tiles::from(out_ch);
                    if tile == Tiles::Robot {
                        robot_pos = map_pos;
                        println!("Found robot {}", char::from(out_ch));
                        robot_dir = match out_ch {
                            b'^' => vec2( 0, -1),
                            b'v' => vec2( 0,  1),
                            b'<' => vec2(-1,  0),
                            b'>' => vec2( 1,  0),
                            _ => panic!("Bad dir")
                        }
                    }
                    *map.tile_mut(map_pos) = tile;
                    map_pos.x += 1;
                }
            }
        }
    }
    println!("Computer {:?} with map {}x{}", state, map.tsize.x, map.tsize.y);
    map.shrink_tiles();
    let path_crosses = map.path_cross();
    let align_sum: i32 = path_crosses.iter()
        .fold(0, |a, p| a + (p.x * p.y));
    // map.debug_tiles(vec2(0, 0));
    println!("Alignment sum {}", align_sum);

    // finds a simple path through all path tiles
    let mut cmds: Vec<Command> = vec![];
    let is_inside = inside_size(map.tsize);
    loop {
        let dirs = [
            (robot_dir, Command::Move(1)),
            (robot_dir.rotate_left(), Command::Left),
            (robot_dir.rotate_right(), Command::Right),
        ];
        let maybe_cmd = dirs.iter().find(|&(dir, _)| {
            let tile_pos = robot_pos + dir;
            is_inside(&tile_pos) &&
                *map.tile(tile_pos) == Tiles::Path
        });
        if let Some(&(dir, cmd)) = maybe_cmd {
            // println!("pos {:?} dir {:?} cmd {:?}", robot_pos, robot_dir, cmd);
            let last = cmds.last_mut();
            match cmd {
                Command::Move(_) => {
                    robot_pos += robot_dir;
                    // merge moves or push
                    if let Some(Command::Move(x)) = last {
                        *x += 1;
                    } else {
                        cmds.push(cmd);
                    }
                }
                Command::Left | Command::Right => {
                    robot_dir = dir;
                    cmds.push(cmd);
                }
            }
        } else {
            break;
        }
    }

    fn split_range(range: &Range<usize>, by: &[Range<usize>], left: &mut Vec<Range<usize>>) {
        let mut rest = range.clone();
        for r in by {
            let before = rest.start..r.start;
            if before.len() > 0 {
                left.push(before);
            }
            rest = r.end..rest.end;
        }
        if rest.len() > 0 {
            left.push(rest);
        }
    }

    fn find_win_sizes(cmds: &Vec<Command>, on_ranges: &Vec<Range<usize>>, depth: i32, stack: &mut Vec<Vec<Range<usize>>>, out: &mut Vec<String>) {
        for win_size in 2..12 {
            let start = on_ranges.first().unwrap().start;
            let pattern = &cmds[start..start + win_size];
            let mut new_ranges: Vec<_> = vec![];
            let mut found: Vec<_> = vec![];
            let mut tmp_found: Vec<_> = vec![];
            for cr in on_ranges.iter() {
                tmp_found.extend(cmds[cr.clone()].windows(win_size)
                    .enumerate()
                    .filter_map(|(i, w)| Some(cr.start + i..cr.start + i + win_size).filter(|_| pattern == w)));

                split_range(&cr, &tmp_found, &mut new_ranges);
                found.append(&mut tmp_found);
            }

            // println!("found {:?}", found);
            stack.push(found);

            if depth - 1 > 0 {
                find_win_sizes(cmds, &new_ranges, depth - 1, stack, out);
            } else {
                fn stack_name(i: usize) -> char {
                    char::from_u32(u32::from('A') + i as u32).unwrap()
                }
                // if everything was covered
                if new_ranges.len() == 0 {
                    // let mut debug_str = String::from_iter(vec!['_'; cmds.len()]);
                    // for (i, rs) in stack.iter().enumerate() {
                    //     for r in rs {
                    //         let with = String::from_iter(vec![stack_name(i); r.len()]);
                    //         debug_str.replace_range(r.clone(), &with);
                    //     }
                    // }
                    // println!("{}", debug_str);

                    let stack_iter = stack.iter()
                        .enumerate()
                        .map(|(i, rs)| rs.iter().map(move |r| (r.start, i)));
                    let main_routine: String = Itertools::intersperse(itertools::kmerge_by(stack_iter,
                        |a: &(usize, usize), b: &(usize, usize)| a.0 < b.0
                        ).map(|(_, i)| stack_name(i)), ',')
                        .collect();
                    out.push(main_routine);

                    out.extend(stack.iter()
                        .map(|rs| {
                            cmds[rs.first().unwrap().clone()].iter().map(|c| {
                                match c {
                                    Command::Move(x) => x.to_string(),
                                    Command::Left => "L".to_string(),
                                    Command::Right => "R".to_string()
                                }
                            }).join(",")
                        })
                    );

                    // check lengths
                    if out.iter().all(|s| s.len() < 20) {
                        break;
                    } else {
                        out.clear();
                    }
                }
            }
            stack.pop();
        }
    }

    // println!("{:?}", cmds);
    let a_ranges = vec![0..cmds.len()];
    let mut stack = vec![];
    let mut progs = vec![];
    find_win_sizes(&cmds, &a_ranges, 3, &mut stack, &mut progs);
    progs.iter_mut().for_each(|s| s.push('\n'));
    println!("Found solution\n{}", progs.concat());

    state = ComputerState::Interrupt;
    let mut newline_counter = 0;
    while ComputerState::Interrupt == state {
        state = comp.run().unwrap();
        if comp.outs.len() >= 1 {
            let out = comp.outs.pop_front().unwrap();
            if out as u32 == u32::from('\n') {
                newline_counter += 1;
                print!("{}", char::from_u32(out as u32).unwrap());
                if newline_counter == 1 {
                    // input programs
                    comp.ins.extend(progs.concat().bytes().map(|b| b as i64));
                } else if newline_counter == 5 {
                    // disable video feed
                    comp.ins.extend("n\n".bytes().map(|b| b as i64));
                }
            } else {
                if newline_counter < 6 + map.tsize.y {
                    print!("{}", char::from_u32(out as u32).unwrap());
                } else {
                    println!("dust_collected {}", out);
                    break;
                }
            }
        }
    }
}
