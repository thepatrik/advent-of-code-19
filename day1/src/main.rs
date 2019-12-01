use std::fs;

static FILENAME: &str = "input/data";

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string(FILENAME).expect("read error");

    let fuel_f = |i: i32| -> i32 { i / 3 - 2 };

    let one = mass(&input, fuel_f);
    println!("part one: {}", one);

    let two = mass(&input, fuel);
    println!("part two: {}", two);

    Ok(())
}

fn mass(input: &str, f: fn(i32) -> i32) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let num: i32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        total += f(num);
    }
    return total;
}

fn fuel(i: i32) -> i32 {
    let f = i / 3 - 2;
    if f < 1 {
        return 0;
    }

    return f + fuel(f);
}

mod tests {
    #[test]
    fn test_fuel() {
        assert_eq!(super::fuel(12), 2);
        assert_eq!(super::fuel(14), 2);
        assert_eq!(super::fuel(1969), 966);
        assert_eq!(super::fuel(100756), 50346);
    }
}
