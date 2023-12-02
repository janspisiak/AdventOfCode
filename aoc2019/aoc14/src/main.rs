use std::{fs, collections::{HashMap, BTreeMap}};

#[derive(Debug)]
struct Reactant<'e> {
    name: &'e str,
    quantity: i64,
}

#[derive(Debug)]
struct Reaction<'a> {
    quantity: i64,
    inputs: Vec<Reactant<'a>>,
}

fn ore_needed(reactions: &HashMap<&str, Reaction>, priority: &BTreeMap<&str, i64>, fuel: i64) -> i64 {
    let mut ore_count = 0;
    let fuel_r = Reactant{ name: "FUEL", quantity: fuel };
    let mut reactants: BTreeMap<i64, Reactant> = BTreeMap::from([(0, fuel_r)]);// HashMap::from([("FUEL", 1)])
    while !reactants.is_empty() {
        // let (prod, prod_needed) = reactants.pop_front().unwrap();
        let (_, product) = reactants.pop_first().unwrap();
        let reaction = reactions.get(product.name).unwrap();
        // integer div_ceil
        let multiplier = (product.quantity + reaction.quantity - 1) / reaction.quantity;
        for reactant in &reaction.inputs {
            if &reactant.name == &"ORE" {
                ore_count += reactant.quantity * multiplier;
            } else {
                let prio = *priority.get(reactant.name).unwrap();
                let maybe = reactants.get_mut(&prio);

                // let maybe_pos = reactants.iter().position(|e| e.0 == *reactant);
                if let Some(existing) = maybe {
                    existing.quantity += reactant.quantity * multiplier;
                } else {
                    reactants.insert(prio, Reactant {
                        name: reactant.name,
                        quantity: reactant.quantity * multiplier,
                    });
                }
            }
        }
        // println!("{:?}", reactants);
    }
    ore_count
}

fn main() {
    let input_path = "aoc14/input.txt";
    let input_str = fs::read_to_string(input_path).expect("Something went wrong reading the file");
    let reactions: HashMap<&str, Reaction> = input_str.lines()
        .fold(HashMap::new(), |mut m, l| {
            let (ins_str, out_str) = l.split_once("=>").unwrap();
            let (quantity, key) = out_str.trim().split_once(' ').unwrap();
            m.insert(key, Reaction {
                quantity: quantity.parse::<i64>().unwrap(),
                inputs: ins_str.split(',').map(|s| {
                    let (q, k) = s.trim().split_once(' ').unwrap();
                    Reactant{ name: k, quantity: q.parse::<i64>().unwrap() }
                }).collect(),
            });
            m
        });
    // println!("{:?}", reactions);

    // we need to know the order of substitutions so we only convert to
    // lower resources once we have all of resource X
    let mut priority: BTreeMap<&str, i64> = BTreeMap::new();
    let mut chems = vec!["FUEL"];
    let mut order = 0;
    while !chems.is_empty() {
        let ch = chems.pop().unwrap();
        priority.insert(ch, order);
        order += 1;
        let maybe_r = reactions.get(ch);
        if let Some(reaction) = maybe_r {
            reaction.inputs.iter().for_each(|r| {
                chems.push(r.name);
            })
        }
    }
    // println!("{:?}", priority);

    // println!("{:?}", products);
    let mut fuel = 1;
    let ore_target = 1_000_000_000_000i64;
    let fuel_result = loop {
        let ore = ore_needed(&reactions, &priority, fuel);
        println!("ore {} fuel {}", ore, fuel);
        if ore > ore_target {
            break fuel - 1;
        } else {
            // new fuel approx given the better ratio estimation
            let fuel_approx = fuel as f64 * (ore_target as f64 / ore as f64).floor();
            // increment at least 1 so we don't iterate too much at the end
            fuel = (fuel + 1).max(fuel_approx as i64);
        }
    };
    println!("fuel_result {}", fuel_result);
}
