use std::io::{self, BufRead};

fn main() -> Result<(), std::io::Error> {
    println!("input number seq");
    let reader = io::stdin();
    let nums: Vec<i32> = reader
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", part_one(nums).unwrap());
    Ok(())
}

fn part_one(mut nums: Vec<i32>) -> Result<(i32), std::io::Error> {
    let mut i: usize = 0;

    loop {
        let op = nums[i];

        match op {
            1 => {
                let pos1 = nums[nums[i + 1] as usize];
                let pos2 = nums[nums[i + 2] as usize];
                let pos3 = nums[i + 3];
                nums[pos3 as usize] = pos1 + pos2;
            }
            2 => {
                let pos1 = nums[nums[i + 1] as usize];
                let pos2 = nums[nums[i + 2] as usize];
                let pos3 = nums[i + 3];
                nums[pos3 as usize] = pos1 * pos2;
            }
            _ => break,
        }
        i = i + 4
    }

    Ok(nums[0])
}

mod tests {
    #[test]
    fn test_part_one() {
        let mut vec = Vec::new();
        let nums = [
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 9, 19, 1, 13, 19, 23, 2, 23, 9,
            27, 1, 6, 27, 31, 2, 10, 31, 35, 1, 6, 35, 39, 2, 9, 39, 43, 1, 5, 43, 47, 2, 47, 13,
            51, 2, 51, 10, 55, 1, 55, 5, 59, 1, 59, 9, 63, 1, 63, 9, 67, 2, 6, 67, 71, 1, 5, 71,
            75, 1, 75, 6, 79, 1, 6, 79, 83, 1, 83, 9, 87, 2, 87, 10, 91, 2, 91, 10, 95, 1, 95, 5,
            99, 1, 99, 13, 103, 2, 103, 9, 107, 1, 6, 107, 111, 1, 111, 5, 115, 1, 115, 2, 119, 1,
            5, 119, 0, 99, 2, 0, 14, 0,
        ];

        // init start seq
        vec.extend(nums.to_vec().into_iter());
        vec[1] = 12;
        vec[2] = 2;
        assert_eq!(super::part_one(vec).unwrap(), 2894520);
    }
}
