use intcode;
use std::collections::HashMap;
use std::fs;
use std::i32;

static FILENAME: &str = "input/data";

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq)]
enum Turn {
    Right,
    Left,
}

#[derive(Debug)]
enum Color {
    Black,
    White,
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: ");
    print(&part_two());
}

fn part_one() -> usize {
    let input = fs::read_to_string(FILENAME).unwrap();
    let grid = process(input, Color::Black);
    grid.len()
}

fn part_two() -> HashMap<(i32, i32), char> {
    let input = fs::read_to_string(FILENAME).unwrap();
    process(input, Color::White)
}

fn process(input: String, color: Color) -> HashMap<(i32, i32), char> {
    let mut dir = Direction::North;
    let mut grid = HashMap::new();

    let mut app = intcode::Intcode::new();
    app.load(input.trim());

    let mut loc = (0, 0);
    &grid.insert(loc, color_to_char(color));

    let out = panel(&grid, loc);
    app.write_to_buff(out);

    loop {
        // Read color code
        let color_code = app.run();
        if app.halted {
            break;
        }

        let color = code_to_color(color_code);
        let chr = color_to_char(color);
        &grid.insert(loc, chr);

        // Read direction code
        let dir_code = app.run();
        if app.halted {
            break;
        }

        let t = code_to_turn(dir_code);
        dir = turn(dir, t);
        loc = nxt_pos(dir, loc);
        let p = panel(&grid, loc);
        app.write_to_buff(p);
    }

    grid
}

fn code_to_color(i: i64) -> Color {
    match i {
        0 => Color::Black,
        1 => Color::White,
        _ => panic!("uknown color: {}", i),
    }
}

fn color_to_char(color: Color) -> char {
    match color {
        Color::Black => '.',
        Color::White => '#',
    }
}

fn code_to_turn(i: i64) -> Turn {
    match i {
        0 => Turn::Left,
        1 => Turn::Right,
        _ => panic!("uknown turn: {}", i),
    }
}

fn char_to_color(c: char) -> Color {
    match c {
        '.' => Color::Black,
        _ => Color::White,
    }
}

fn color_to_code(color: Color) -> i64 {
    match color {
        Color::Black => 0,
        Color::White => 1,
    }
}

fn turn(dir: Direction, turn: Turn) -> Direction {
    match dir {
        Direction::North => {
            if turn == Turn::Left {
                return Direction::West;
            }
            Direction::East
        }
        Direction::South => {
            if turn == Turn::Left {
                return Direction::East;
            }
            Direction::West
        }
        Direction::East => {
            if turn == Turn::Left {
                return Direction::North;
            }
            Direction::South
        }
        Direction::West => {
            if turn == Turn::Left {
                return Direction::South;
            }
            Direction::North
        }
    }
}

fn nxt_pos(dir: Direction, loc: (i32, i32)) -> (i32, i32) {
    match dir {
        Direction::North => (loc.0, loc.1 - 1),
        Direction::South => (loc.0, loc.1 + 1),
        Direction::East => (loc.0 + 1, loc.1),
        Direction::West => (loc.0 - 1, loc.1),
    }
}

fn panel(grid: &HashMap<(i32, i32), char>, loc: (i32, i32)) -> i64 {
    match grid.get(&loc) {
        Some(c) => color_to_code(char_to_color(*c)),
        None => color_to_code(Color::Black),
    }
}

pub fn print(grid: &HashMap<(i32, i32), char>) {
    let mut west_bound: i32 = i32::MAX;
    let mut east_bound = i32::MIN;
    let mut north_bound = i32::MAX;
    let mut south_bound = i32::MIN;

    for loc in grid.keys().into_iter() {
        if loc.0 < west_bound {
            west_bound = loc.0;
        }
        if loc.0 > east_bound {
            east_bound = loc.0;
        }
        if loc.1 < north_bound {
            north_bound = loc.1;
        }
        if loc.1 > south_bound {
            south_bound = loc.1;
        }
    }

    for r in north_bound..south_bound + 1 {
        let mut row = String::new();
        for c in west_bound..east_bound + 1 {
            match grid.get(&(c, r)) {
                Some(v) => {
                    row.push(*v);
                }
                None => row.push(color_to_char(Color::Black)),
            }
        }
        println!("{}", row);
    }
}

mod tests {
    #[test]
    fn tests() {
        assert_eq!(super::part_one(), 2088);
    }
}
