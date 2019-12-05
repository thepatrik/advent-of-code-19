fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input/data")
        .expect("Failed to read input/data");

    let vec = parse_input(&input)
        .expect("Failed to parse input file");

    println!("part one: {}", part_one(&vec));
    println!("part two: {}", part_two(&vec));

    Ok(())
}

fn part_one(nums: &[usize]) -> usize {
    process(nums, 12, 2)
}

fn part_two(nums: &[usize]) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            if process(nums, noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("Unreachable")
}

fn process(nums: &[usize], noun: usize, verb: usize) -> usize {
    let mut mem = nums.to_vec();
    let mut i: usize = 0;

    mem[1] = noun;
    mem[2] = verb;

    loop {
        let op = mem[i];

        match op {
            1 => {
                let pos1 = mem[mem[i + 1]];
                let pos2 = mem[mem[i + 2]];
                let pos3 = mem[i + 3];
                mem[pos3] = pos1 + pos2;
            }
            2 => {
                let pos1 = mem[mem[i + 1]];
                let pos2 = mem[mem[i + 2]];
                let pos3 = mem[i + 3];
                mem[pos3] = pos1 * pos2;
            }
            _ => break,
        }
        i = i + 4;
    }

    mem[0]
}

fn parse_input(input: &str) -> Option<Vec<usize>> {
    input.trim().split(",").map(|s| s.parse().ok()).collect()
}

mod tests {
    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("input/data")
            .expect("Failed to read input/data");

        let vec = super::parse_input(&input)
            .expect("Failed to parse input file");

        assert_eq!(super::part_one(&vec), 2894520);
        assert_eq!(super::part_two(&vec), 9342);
    }
}
