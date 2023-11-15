use std::{fs, iter};

fn part_one(input: &Vec<i32>) {
    let mut in_vec = input.clone();
    let mut out_vec = vec![];
    for _step in 0..100 {
        out_vec.resize(in_vec.len(), 0);
        for y in 0..in_vec.len() {
            let sum: i32 = in_vec.iter()
                .enumerate()
                .map(|(x, v)| {
                    let coef = match ((x + 1) / (y + 1)) as i32 % 4 {
                        0 => 0,
                        1 => 1,
                        2 => 0,
                        3 => -1,
                        _ => panic!("bad arithmetic")
                    };
                    v * coef
                })
                .sum();
            // only last digit
            out_vec[y] = sum.abs() % 10;
        }
        // println!("phase {} val {:?}", step, out_vec);
        in_vec.clear();
        in_vec.append(&mut out_vec);
    }
    println!("val {:?}", in_vec);
}

// uses the knowledge that the offset is very close to the end
// 1) we don't need any values before the offset (since they don't affect the result)
// 2) the coef matrix will be filled with 1 in upper right triangle,
//    so we can reverse partial sum
fn part_two(input: &Vec<i32>, msg_offset: usize) {
    let mut in_vec = input.clone();
    let total_len = in_vec.len() * 10000;
    let work_len = total_len - msg_offset;

    let mut out_vec = vec![];
    out_vec.extend(iter::repeat(&in_vec)
        .take(10000)
        .flat_map(|x| x.iter())
        .skip(msg_offset)
    );
    in_vec.clear();
    in_vec.append(&mut out_vec);
    out_vec.clear();

    println!("total_len {} work_len {} msg_offset {} in_vec {}", total_len, work_len, msg_offset, in_vec.len());

    for _step in 0..100 {
        out_vec.resize(in_vec.len(), 0);
        let mut sum = 0;
        for y in (0..in_vec.len()).rev() {
            sum += in_vec[y];
            // only last digit
            out_vec[y] = sum.abs() % 10;
        }
        // println!("phase {} val {:?}", step, out_vec);
        in_vec.clear();
        in_vec.append(&mut out_vec);
    }
    println!("val {:?}", &in_vec[0..8]);
    // 69732268
}

fn main() {
    let input_path = "aoc16/input.txt";
    let input_str = fs::read_to_string(input_path).expect("Something went wrong reading the file");
    let in_vec: Vec<_> = input_str.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    let msg_offset: usize = (&input_str[0..7]).parse().unwrap();

    // part_one(&in_vec);
    part_two(&in_vec, msg_offset);
}
