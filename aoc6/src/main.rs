use std::{fs, collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
	let input_path = "aoc6/input.txt";
	let input_str = fs::read_to_string(input_path)
		.expect("Something went wrong reading the file");

	let orbit_vec: Vec<_> = input_str
		.split('\n')
		.filter_map(|line| line.split_once(')'))
		.collect();

    let mut outgoing: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut incoming: HashMap<&str, Option<&str>> = HashMap::new();
    for (from, to) in orbit_vec {
        // build outgoing map
        outgoing.entry(from).and_modify(|nodes| nodes.push(to)).or_insert(Vec::from([to]));
        // build incoming map
        incoming.entry(from).or_insert(None);
        incoming.insert(to, Some(from));
    }

    let center = incoming.iter().find_map(|(key, &val)| if val == None { Some(*key) } else { None });

    println!("center {}", center.unwrap());

    let mut to_visit = Vec::from([(center.unwrap(), 0)]);
    let mut sum = 0;
    let mut me_path: Vec<&str> = Vec::new();
    let mut san_path: Vec<&str> = Vec::new();
    while !to_visit.is_empty() {
        let (node, depth) = to_visit.pop().unwrap();
        let out_nodes = outgoing.get(node);
        match out_nodes {
            Some(out_nodes) => out_nodes.iter().for_each(|d| to_visit.push((*d, depth + 1))),
            None => (),
        }

        let get_path = |node| -> Vec<&str> {
            let mut path: Vec<&str> = Vec::new();
            let mut prev = incoming.get(node).unwrap();
            while let Some(parent) = *prev {
                path.push(parent);
                prev = incoming.get(parent).unwrap();
            }
            path
        };

        match node {
            "YOU" => me_path = get_path(node),
            "SAN" => san_path = get_path(node),
            _ => ()
        }
        sum += depth;
    }

    me_path.reverse();
    san_path.reverse();
    let diff_depth = me_path.iter()
        .zip(san_path.iter())
        .position(|(&m, &s)| m != s).unwrap_or(me_path.len());

    println!("me_path {} san_path {} diff_depth {}", me_path.len(), san_path.len(), diff_depth);
    // println!("me_path {:#?} san_path {:#?}", me_path.iter().take(4), san_path.iter().take(4));
    println!("Sum {}", sum);
    println!("Path {}", me_path.len() + san_path.len() - 2 * diff_depth);
    Ok(())
}
