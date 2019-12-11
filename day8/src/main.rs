static FILENAME: &str = "input/data";

fn main() {
    println!("part one: {}", part_one(25, 6));
    println!("part two: \n{}", part_two(25, 6));
}

pub fn part_one(w: usize, h: usize) -> usize {
    let input = std::fs::read_to_string(FILENAME).expect("Failed to read input data");
    let layer_len = w * h;
    let mut res = 0;
    let mut min = std::usize::MAX;
    let (mut zeroes, mut ones, mut twos) = (0, 0, 0);
    let mut i = 0;

    for c in input.trim().chars() {
        i += 1;

        match c.to_digit(10).unwrap() {
            0 => zeroes += 1,
            1 => ones += 1,
            2 => twos += 1,
            _ => continue,
        }

        if i % layer_len == 0 {
            // end of layer
            if zeroes < min {
                res = ones * twos;
                min = zeroes;
            }

            zeroes = 0;
            ones = 0;
            twos = 0;
        }
    }

    res
}

fn part_two(w: usize, h: usize) -> String {
	let mut input = std::fs::read_to_string(FILENAME).expect("Failed to read input data");
	input = input.replace("0", ".");	
	input = input.replace("1", "#");	
    input = input.replace("2", " ");	
    
    let layer_len = w * h;

	let mut canvas = vec![' '; layer_len];
	let mut i = 0;
	for ch in input.chars() {
		if canvas[i] == ' ' {
			canvas[i] = ch;
		}
		i += 1;
		if i % layer_len == 0 {
			i = 0;
		}	
	}

	for n in 1..7 {
		canvas.insert(n * 25 + n - 1, '\n');
	}
	canvas.iter().collect()
}

mod tests {
    #[test]
    fn tests() {
        assert_eq!(super::part_one(25, 6), 1742);
    }
}
