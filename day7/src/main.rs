use intcode;
use std::collections::HashSet;
use std::fs;

static FILENAME: &str = "input/data";

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}

pub fn part_one() -> i64 {
    let input = fs::read_to_string(FILENAME).unwrap();
    let mut max = 0;

    for i in 0..5 {
        for j in 0..5 {
            for k in 0..5 {
                for l in 0..5 {
                    for m in 0..5 {
                        let phase_seq: HashSet<i64> = vec![i, j, k, l, m].into_iter().collect();
                        if phase_seq.len() == 5 {
                            // App A
                            let mut app_a = intcode::Intcode::new();
                            app_a.load(input.trim());
                            app_a.write_to_buff(i);
                            app_a.write_to_buff(0);
                            let thrust = app_a.run();

                            // App B
                            let mut app_b = intcode::Intcode::new();
                            app_b.load(input.trim());
                            app_b.write_to_buff(j);
                            app_b.write_to_buff(thrust);
                            let thrust = app_b.run();

                            // App C
                            let mut app_c = intcode::Intcode::new();
                            app_c.load(input.trim());
                            app_c.write_to_buff(k);
                            app_c.write_to_buff(thrust);
                            let thrust = app_c.run();

                            // App D
                            let mut app_d = intcode::Intcode::new();
                            app_d.load(input.trim());
                            app_d.write_to_buff(l);
                            app_d.write_to_buff(thrust);
                            let thrust = app_d.run();

                            // App D
                            let mut app_e = intcode::Intcode::new();
                            app_e.load(input.trim());
                            app_e.write_to_buff(m);
                            app_e.write_to_buff(thrust);
                            let thrust = app_e.run();

                            if thrust > max {
                                max = thrust
                            }
                        }
                    }
                }
            }
        }
    }

    max
}

fn part_two() -> i64 {
    let input = fs::read_to_string(FILENAME).unwrap();
    let mut max = 0;

    for i in 5..10 {
        for j in 5..10 {
            for k in 5..10 {
                for l in 5..10 {
                    for m in 5..10 {
                        let phases = vec![i, j, k, l, m];
                        let set: HashSet<i64> = vec![i, j, k, l, m].into_iter().collect();
                        if set.len() != 5 {
                            continue;
                        }

                        let thrust = amp_loop(input.trim(), phases);
                        if thrust > max {
                            max = thrust
                        }
                    }
                }
            }
        }
    }

    max
}

pub fn amp_loop(input: &str, seq: Vec<i64>) -> i64 {
    let mut app_a = intcode::Intcode::new();
    app_a.init(input, seq[0]);
    let mut app_b = intcode::Intcode::new();
    app_b.init(input, seq[1]);
    let mut app_c = intcode::Intcode::new();
    app_c.init(input, seq[2]);
    let mut app_d = intcode::Intcode::new();
    app_d.init(input, seq[3]);
    let mut app_e = intcode::Intcode::new();
    app_e.init(input, seq[4]);

    let mut a_feedback = 0;
    while app_e.state != intcode::State::Halted {
        let mut out = process(&mut app_a, a_feedback);
        out = process(&mut app_b, out);
        out = process(&mut app_c, out);
        out = process(&mut app_d, out);
        out = process(&mut app_e, out);
        a_feedback = out;
    }

    app_e.output
}

fn process(app: &mut intcode::Intcode, input: i64) -> i64 {
    app.write_to_buff(input);
    app.run()
}

mod tests {
    #[test]
    fn test() {
        assert_eq!(super::part_one(), 118936);
        assert_eq!(super::part_two(), 57660948);
    }
}
