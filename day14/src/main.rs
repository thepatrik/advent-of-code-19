use std::collections::HashMap;
use std::fs;

static FILENAME: &str = "input/data";

#[derive(Clone)]
struct Chemical {
    name: String,
    units: usize,
}

struct Reaction {
    produces: usize,
    requires: Vec<Chemical>,
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}

fn part_one() -> usize {
    let reactions = parse_input(FILENAME);

    calc_ore(
        String::from("FUEL"),
        1,
        &reactions,
        &mut HashMap::<String, usize>::new(),
    )
}

fn part_two() -> usize {
    let reactions = parse_input(FILENAME);
    max_fuel(&reactions)
}

fn calc_ore(
    chem: String,
    wanted_units: usize,
    reactions: &HashMap<String, Reaction>,
    spare: &mut HashMap<String, usize>,
) -> usize {
    if chem == "ORE" {
        return wanted_units;
    }

    let excess = spare.entry(chem.to_string()).or_default();
    if *excess >= wanted_units {
        *excess -= wanted_units;
        return 0;
    }

    let mut quantity = wanted_units;

    quantity -= *excess;
    *excess = 0;

    let reaction = reactions.get(&chem).unwrap();
    let units = ((quantity as f64) / (reaction.produces as f64)).ceil() as usize;
    let new_spare = reaction.produces * units - quantity;
    *spare.entry(chem.to_string()).or_default() += new_spare;

    // sum up all required inputs
    let mut sum = 0;
    for req in reaction.requires.clone() {
        sum += calc_ore(req.name, req.units * units, &reactions, spare);
    }

    sum
}

fn max_fuel(reactions: &HashMap<String, Reaction>) -> usize {
    let min = calc_ore(
        String::from("FUEL"),
        1,
        &reactions,
        &mut HashMap::<String, usize>::new(),
    );
    let max = 1000000000000;

    let mut l = 0;
    let mut r: usize = min * 7;

    while l <= r {
        let m = (l + r) / 2;
        let sum = calc_ore(
            String::from("FUEL"),
            m,
            &reactions,
            &mut HashMap::<String, usize>::new(),
        );

        if sum < max {
            l = m + 1;
        } else if sum > max {
            r = m - 1;
        } else {
            return m;
        }
    }
    r
}

fn parse_input(filename: &str) -> HashMap<String, Reaction> {
    let mut reactions = HashMap::<String, Reaction>::new();
    let file_input = fs::read_to_string(filename).unwrap();

    let split_f = |item: &str| {
        let mut rsplit = item.split(" ");
        let units = rsplit.next().unwrap().parse::<usize>().unwrap();
        let chem = rsplit.next().unwrap().to_owned();
        (chem, units)
    };

    file_input.trim().split("\n").for_each(|line| {
        let mut split = line.split(" => ");
        let lhs = split.next().unwrap();
        let rhs = split.next().unwrap();

        let (name, units) = split_f(rhs);
        let requires: Vec<Chemical> = lhs
            .split(", ")
            .map(split_f)
            .map(|(name, units)| Chemical { name, units })
            .collect();

        reactions.insert(
            name.clone(),
            Reaction {
                produces: units,
                requires,
            },
        );
    });

    reactions
}

mod tests {
    #[test]
    fn tests() {
        assert_eq!(super::part_one(), 2486514);
        assert_eq!(super::part_two(), 998536);
    }
}
