pub fn part_one() -> i32 {
    process(vec![
        Box::new(|s: String| !s.contains("0")),
        Box::new(|s: String| {
            (0..s.chars().count() - 1).all(|i| s.chars().nth(i) <= s.chars().nth(i + 1))
        }),
        Box::new(|s: String| {
            (0..s.chars().count() - 1).any(|i| s.chars().nth(i) == s.chars().nth(i + 1))
        }),
    ])
}

pub fn part_two() -> i32 {
    let repeats = Box::new(|s: String| {
        let mut i: usize = 0;
        let mut matches: usize = 0;

        for c in s.chars().take(s.chars().count() - 1) {
            if (c as usize) == (s.chars().nth(i + 1).unwrap() as usize) {
                matches += 1;
            } else {
                if matches == 1 {
                    return true;
                }
                matches = 0
            }
            i += 1;
        }
        matches == 1
    });

    process(vec![
        Box::new(|s: String| !s.contains("0")),
        Box::new(|s: String| {
            (0..s.chars().count() - 1).all(|i| s.chars().nth(i) <= s.chars().nth(i + 1))
        }),
        repeats,
    ])
}

fn process(v: Vec<Box<dyn Fn(String) -> bool>>) -> i32 {
    let mut count = 0;
    for pwd in 372304..847060 + 1 {
        if v.iter().all(|f| f(pwd.to_string())) {
            count += 1
        }
    }
    count
}

mod tests {
    #[test]
    fn test() {
        assert_eq!(super::part_one(), 475);
        assert_eq!(super::part_two(), 297);
    }
}
