use intcode;
use std::fs;

static FILENAME: &str = "input/data";

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}

fn part_one() -> i64 {
    let input = fs::read_to_string(FILENAME).unwrap();
    process(input.trim(), 1)
}

fn part_two() -> i64 {
    let input = fs::read_to_string(FILENAME).unwrap();
    process(input.trim(), 2)
}

fn process(input: &str, i: i64) -> i64 {
    let mut app = intcode::Intcode::new();
    app.load(input);
    app.write_to_buff(i);

    loop {
        let out = app.run();
        if app.state == intcode::State::Halted {
            return out;
        }
    }
}

mod tests {
    #[test]
    fn tests() {
        assert_eq!(super::part_one(), 2399197539);
        assert_eq!(super::part_two(), 35106);
    }
}
