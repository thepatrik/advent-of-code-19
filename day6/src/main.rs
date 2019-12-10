use std::collections::HashMap;

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}

pub fn part_one() ->i32 {
    let input = std::fs::read_to_string("input/data")
        .expect("Something went wrong reading the file");

    let orbits = parse_orbits(&input);

    let mut c: i32 = 0;
    for (k, _) in &orbits {
        c += count(k, &orbits);
    }

    c
}

pub fn part_two() ->i32 {
    let input = std::fs::read_to_string("input/data")
        .expect("Something went wrong reading the file");

    let orbits = parse_orbits(&input);

    let from_key = "YOU";
    let to_key = "SAN";

    let mut curr_key = String::from(from_key);
    let mut dest_key = String::from(to_key);

    loop {
        if !orbits.contains_key(&curr_key) {
            dest_key = orbits.get(&dest_key).unwrap().to_string();
            curr_key = String::from(from_key);
            continue;
        }
        let nxt_key = orbits.get(&curr_key).unwrap();
        let steps = steps_to_key(&nxt_key, &dest_key, &orbits);
        if steps > -1 {
            let swap_key = String::from(to_key);
            let turnaround = steps_to_key(&swap_key, &dest_key, &orbits);
            return steps + turnaround + 1;
        }
        curr_key = nxt_key.to_string();
    }
}

fn steps_to_key(key: &String, to_key: &String, map: &HashMap<String, String>) -> i32 {
    if !map.contains_key(key) {
        return -1;
    }
    if map.get(key).unwrap() == to_key {
        return 0;
    }

    let nxt_key = map.get(key).unwrap();

    let mut res = steps_to_key(nxt_key, to_key, map);
    if res > -1 {
        res+=1
    }

    res
}

fn count(key: &String, map: &HashMap<String, String>) -> i32 {
    if !map.contains_key(key) {
        return 0
    }

    let nxt_key = map.get(key).unwrap();

    return count(nxt_key, map) + 1;
}

fn parse_orbits(input: &str) -> HashMap<String, String> {
    let mut orbits = HashMap::new();
    
    input.lines()
        .map(|kv| kv.split(')').collect::<Vec<&str>>())
        .map(|vec| (vec[1].to_string(), vec[0].to_string()))
        .fold((), |_, (k, v)| {
            orbits.insert(k, v);
        });

    orbits
}

mod tests {
    #[test]
    fn test() {
        assert_eq!(super::part_one(), 453028);
        assert_eq!(super::part_two(), 562);
    }
}
