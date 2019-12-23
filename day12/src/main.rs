extern crate num;

use num::integer::lcm;

#[derive(Copy, Clone, Debug)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Clone, Debug)]
struct Moon {
    pos: Position,
    vel: Position,
}

impl Moon {
    fn energy(&self) -> usize {
        let pot = self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs();
        let kin = self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs();
        (pot * kin) as usize
    }

    fn new(x: isize, y: isize, z: isize) -> Moon {
        Moon {
            pos: Position { x: x, y: y, z: z },
            vel: Position { x: 0, y: 0, z: 0 },
        }
    }
}

fn main() {
    let mut moons = Vec::new();
    moons.push(Moon::new(-7, -1, 6));
    moons.push(Moon::new(6, -9, -9));
    moons.push(Moon::new(-12, 2, -7));
    moons.push(Moon::new(4, -17, -12));

    println!("part one: {}", part_one(&mut moons, 1000));
    println!("part two: {}", part_two(&mut moons));
}

fn part_one(moons: &mut Vec<Moon>, steps: usize) -> usize {
    for _ in 0..steps {
        add_gravity(moons);
        add_velocity(moons);
    }

    moons.iter().map(|moon| moon.energy()).sum()
}

fn part_two(moons: &mut Vec<Moon>) -> usize {
    let x_init_state = x_state(&moons);
    let y_init_state = y_state(&moons);
    let z_init_state = z_state(&moons);

    let mut pos = Position { x: 0, y: 0, z: 0 };
    let mut step: isize = 0;

    loop {
        step += 1;

        add_gravity(moons);
        add_velocity(moons);

        if pos.x == 0 && x_state(&moons) == x_init_state {
            pos.x = step;
        }
        if pos.y == 0 && y_state(&moons) == y_init_state {
            pos.y = step;
        }
        if pos.z == 0 && z_state(&moons) == z_init_state {
            pos.z = step;
        }

        if pos.x != 0 && pos.y != 0 && pos.z != 0 {
            break;
        }
    }

    lcm(pos.x as usize, lcm(pos.y as usize, pos.z as usize))
}

fn add_gravity(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            moons[i].vel.x += (moons[j].pos.x - moons[i].pos.x).signum();
            moons[i].vel.y += (moons[j].pos.y - moons[i].pos.y).signum();
            moons[i].vel.z += (moons[j].pos.z - moons[i].pos.z).signum();
        }
    }
}

fn add_velocity(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        moons[i].pos.x += moons[i].vel.x;
        moons[i].pos.y += moons[i].vel.y;
        moons[i].pos.z += moons[i].vel.z;
    }
}

fn x_state(moons: &[Moon]) -> [isize; 8] {
    [
        moons[0].pos.x,
        moons[0].vel.x,
        moons[1].pos.x,
        moons[1].vel.x,
        moons[2].pos.x,
        moons[2].vel.x,
        moons[3].pos.x,
        moons[3].vel.x,
    ]
}

fn y_state(moons: &[Moon]) -> [isize; 8] {
    [
        moons[0].pos.y,
        moons[0].vel.y,
        moons[1].pos.y,
        moons[1].vel.y,
        moons[2].pos.y,
        moons[2].vel.y,
        moons[3].pos.y,
        moons[3].vel.y,
    ]
}

fn z_state(moons: &[Moon]) -> [isize; 8] {
    [
        moons[0].pos.z,
        moons[0].vel.z,
        moons[1].pos.z,
        moons[1].vel.z,
        moons[2].pos.z,
        moons[2].vel.z,
        moons[3].pos.z,
        moons[3].vel.z,
    ]
}

mod tests {
    #[test]
    fn tests() {
        let mut moons = Vec::new();
        moons.push(super::Moon::new(-7, -1, 6));
        moons.push(super::Moon::new(6, -9, -9));
        moons.push(super::Moon::new(-12, 2, -7));
        moons.push(super::Moon::new(4, -17, -12));

        assert_eq!(super::part_one(&mut moons, 1000), 11384);
        assert_eq!(super::part_two(&mut moons), 452582583272768);
    }
}
