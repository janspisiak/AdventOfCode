use std::{fs, str, collections::HashMap};
use itertools::Itertools;

fn count(l: &Vec<char>) -> i32 {
	return l.iter()
        .fold(0, |s, i| {
            if *i == '0' { s + 1 } else { s }
        });
}

struct ImageLayer {
    data: Vec<char>,
    hist: HashMap<char, i32>,
}

fn main() {
	let input_path = "aoc8/input.txt";
	let input_str = fs::read_to_string(input_path)
		.expect("Something went wrong reading the file");

    let (w, h) = (25, 6);
    let layers: Vec<_> = input_str
        .chars()
        .chunks(w*h)
        .into_iter()
        .map(|ch| {
            let data = ch.collect::<Vec<_>>();
            let char_map = data
                .iter()
                .fold(HashMap::new(), |mut acc, &c| {
                    *acc.entry(c).or_insert(0) += 1;
                    acc
                });
            ImageLayer {
                data: data,
                hist: char_map,
            }
        })
        .collect();

    // layers.sort_by(|a, b| {
    //     a.hist.get(&'0').unwrap_or(&0).cmp(b.hist.get(&'0').unwrap_or(&0))
    // });
	// println!("part one: {}", layers[0].hist.get(&'1').unwrap() * layers[0].hist.get(&'2').unwrap());

    let mut result = Vec::new();
    for i in (0..w*h) {
        let mut color = '0';
        for l in layers.iter() {
            color = l.data[i];
            if color != '2' { break };
        }
        result.push(color);
    }

    result
        .chunks(w)
        .map(|ch| ch.iter().collect::<String>())
        .for_each(|r| println!("{}", r));
}