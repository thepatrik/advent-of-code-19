static FILENAME: &str = "input/data";

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}

pub fn part_one() -> usize {
    let input = std::fs::read_to_string(FILENAME).expect("failed to read input/data");
    let codes = parse_input(&input).expect("failed to parse input file");
    diagnostic(&codes, 1)
}

pub fn part_two() -> usize {
    let input = std::fs::read_to_string(FILENAME).expect("failed to read input/data");
    let codes = parse_input(&input).expect("failed to parse input file");
    diagnostic(&codes, 5)
}

fn parse_input(s: &str) -> Option<Vec<isize>> {
    s.trim().split(",").map(|s| s.parse().ok()).collect()
}

fn decode_instr(vec: &[isize], instr: usize, idx: u32) -> isize {
    let code = vec[instr];
    let arg = vec[instr + 1 + idx as usize];
    if (code % 10_isize.pow(idx + 3)) / 10_isize.pow(idx + 2) == 1 {
        return arg;
    }
    vec[arg as usize]
}

fn diagnostic(opcodes: &[isize], system_id: isize) -> usize {
    let mut vec = opcodes.to_vec();
    let mut instr = 0;
    loop {
        match vec[instr] % 100 {
            1 => {
                let param_0 = decode_instr(&vec, instr, 0);
                let param_1 = decode_instr(&vec, instr, 1);
                let param_2 = vec[instr + 3];
                vec[param_2 as usize] = param_0 + param_1;
                instr += 4;
            }
            2 => {
                let param_0 = decode_instr(&vec, instr, 0);
                let param_1 = decode_instr(&vec, instr, 1);
                let param_2 = vec[instr + 3];
                vec[param_2 as usize] = param_0 * param_1;
                instr += 4;
            }
            3 => {
                let param_0 = vec[instr + 1];
                vec[param_0 as usize] = system_id;
                instr += 2;
            }
            4 => {
                let param_0 = decode_instr(&vec, instr, 0);
                let chars: Vec<u8> = param_0.to_string().bytes().collect();
                let diagnostic_str = std::str::from_utf8(&chars).unwrap();
                let diagnostic_num: usize = diagnostic_str.parse().unwrap();
                if diagnostic_num > 0 {
                    return diagnostic_num;
                }
                instr += 2;
            }
            5 => {
                let param_0 = decode_instr(&vec, instr, 0);
                if param_0 != 0 {
                    instr = decode_instr(&vec, instr, 1) as usize;
                } else {
                    instr += 3;
                }
            }
            6 => {
                let param_0 = decode_instr(&vec, instr, 0);
                if param_0 == 0 {
                    instr = decode_instr(&vec, instr, 1) as usize;
                } else {
                    instr += 3;
                }
            }
            7 => {
                let param_0 = decode_instr(&vec, instr, 0);
                let param_1 = decode_instr(&vec, instr, 1);
                let param_2 = vec[instr + 3];
                if param_0 < param_1 {
                    vec[param_2 as usize] = 1;
                } else {
                    vec[param_2 as usize] = 0;
                }
                instr += 4
            }
            8 => {
                let param_0 = decode_instr(&vec, instr, 0);
                let param_1 = decode_instr(&vec, instr, 1);
                let param_2 = vec[instr + 3];
                if param_0 == param_1 {
                    vec[param_2 as usize] = 1;
                } else {
                    vec[param_2 as usize] = 0;
                }
                instr += 4
            }
            _ => panic!("unknown instruction"),
        }
    }
}

mod tests {
    #[test]
    fn test() {
        assert_eq!(super::part_one(), 13210611);
        assert_eq!(super::part_two(), 584126);
    }
}
