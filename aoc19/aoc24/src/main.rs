use std::{fs, collections::HashSet};

use euclid::{Vector2D, UnknownUnit, vec2};
use bitvec::{view::BitView, order::Lsb0};
type StateType = u32;
type Vec2i = Vector2D<i32, UnknownUnit>;

fn print_state(state: StateType, width: usize) {
    state.view_bits::<Lsb0>()
        .chunks(width)
        .take(width)
        .for_each(|c| {
            c.iter()
                .map(|b| if *b {'#'} else {'.'})
                .for_each(|c| print!("{c}"));
            println!();
        });
    println!();
}

const ALL_DIRS: &[Vec2i] = &[
    vec2( 0, -1),
    vec2( 0,  1),
    vec2(-1,  0),
    vec2( 1,  0),
];

fn neighbours() -> impl Iterator<Item = (Vec2i, i32))> {
    (1..5).into_iter().map(|x| Dir::from(x))
}

fn inside_size(size: Vec2i) -> impl Fn(&Vector2D<i32, UnknownUnit>) -> bool {
    move |pos: &Vec2i| {
        !pos.lower_than(vec2(0, 0)).any() &&
        !pos.greater_than(vec2(size.x - 1, size.y - 1)).any()
    }
}

fn main() {
    let input_path = "aoc24/input.txt";
    let input_str = fs::read_to_string(input_path).expect("Something went wrong reading the file");
    let init_state = input_str.lines()
        .flat_map(|l| l.chars())
        .enumerate()
        .fold(0u32, |a, (i, c)| {
            let f = match c {
                '#' => 1 << (i as u32),
                '.' | _ => 0,
            };
            a | f
        });

    let width = 5;
    let iwidth = width as i32;
    let is_inside = inside_size(vec2(iwidth, iwidth));
    print_state(init_state, width);
    let mut state = init_state;
    let mut past_states = HashSet::from([init_state]);
    loop {
        let mut next_state: StateType = 0;
        let next_bits = next_state.view_bits_mut::<Lsb0>();
        for (i, b) in state.view_bits::<Lsb0>().iter()
            .take(width * width)
            .enumerate()
        {
            let bi = i as i32;
            let pos = vec2(bi % iwidth, bi / iwidth);
            let bits = state.view_bits::<Lsb0>();
            let bug_count = ALL_DIRS.iter()
                .filter_map(|dir| {
                    let np = pos + dir;
                    let ni = np.y * iwidth + np.x;
                    if is_inside(&np) && ni >= 0 {
                        bits.get(ni as usize).filter(|b| **b)
                    } else {
                        None
                    }
                })
                .count();
            // print!("{bug_count}");
            // if i % width == 4 {
            //     println!();
            // }
            let n = if *b {
                match bug_count {
                    1 => true,
                    _ => false,
                }
            } else {
                match bug_count {
                    1 | 2 => true,
                    _ => false,
                }
            };
            next_bits.set(i, n);
        }
        state = next_state;
        if past_states.contains(&state) {
            break;
        } else {
            past_states.insert(state);
        }
    }
    print_state(state, width);
    println!("biodiversity {}", state as u32);
}
