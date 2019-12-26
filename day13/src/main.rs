use intcode;
use std::cell::Cell;
use std::collections::HashMap;
use std::fs;

static FILENAME: &str = "input/data";

#[derive(Clone, PartialEq)]
enum Tile {
    Empty,      // No game object appears in this tile.
    Wall,       // Walls are indestructible barriers.
    Block,      // Blocks can be broken by the ball.
    Horizontal, // The paddle is indestructible.
    Ball,       // The ball moves diagonally and bounces off objects.
}

fn to_tile(i: i64) -> Tile {
    match i {
        0 => Tile::Empty,
        1 => Tile::Wall,
        2 => Tile::Block,
        3 => Tile::Horizontal,
        4 => Tile::Ball,
        _ => panic!("uknown id: {}", i),
    }
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}

fn part_one() -> usize {
    let input = fs::read_to_string(FILENAME).unwrap();
    let game = parse_game(input);
    game.iter().filter(|kv| *kv.1 == Tile::Block).count()
}

fn part_two() -> usize {
    let input = fs::read_to_string(FILENAME).unwrap();
    play_game(input)
}

fn parse_game(input: String) -> HashMap<(usize, usize), Tile> {
    let mut app = intcode::Intcode::new();
    app.load(input.trim());

    let mut game = HashMap::new();

    loop {
        let x = app.run();
        if app.state == intcode::State::Halted {
            break;
        }

        let y = app.run();
        let n = app.run();
        game.insert((x as usize, y as usize), to_tile(n));
    }

    game
}

fn play_game(input: String) -> usize {
    let mut app = intcode::Intcode::new();
    app.load(input.trim());
    app.write(0, 2);

    let mut score = 0;
    let x_ball: Cell<u32> = Cell::new(0);
    let x_paddle: Cell<u32> = Cell::new(0);

    loop {
        let x = app.run();
        match app.state {
            intcode::State::Halted => break,
            intcode::State::Waiting => {
                let joy_move = (x_ball.get() as isize - x_paddle.get() as isize).signum();
                app.write_to_buff(joy_move as i64);
                continue;
            }
            _ => {}
        }

        let y = app.run();
        let n = app.run();

        if x == -1 && y == 0 {
            score = n;
            continue;
        }

        match to_tile(n) {
            Tile::Ball => {
                x_ball.set(x as u32);
            }
            Tile::Horizontal => {
                x_paddle.set(x as u32);
            }
            _ => {}
        }
    }

    score as usize
}

mod tests {
    #[test]
    fn tests() {
        assert_eq!(super::part_one(), 301);
        assert_eq!(super::part_two(), 14096);
    }
}
