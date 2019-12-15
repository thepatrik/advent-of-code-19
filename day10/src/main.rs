static FILENAME: &str = "input/data";

fn main() {
    println!("part one: {}", part_one());
    println!("part one: {}", part_two());
}

fn part_one() -> usize {
    let input = std::fs::read_to_string(FILENAME).expect("Could not read file");
    let parsed = parse(&input);
    let map = asteroid_map(&parsed);
    let pos = starting_position(&map);
    map[pos.0][pos.1]
}

fn part_two() -> usize {
    let input = std::fs::read_to_string(FILENAME).expect("Could not read file");
    let parsed = parse(&input);
    let map = asteroid_map(&parsed);
    let pos = starting_position(&map);

    let v0: (isize, isize) = (0, 0 - pos.1 as isize);

    let mut vaporized = Vec::new();
    let mut vec = parsed.clone();
    while vaporized.len() < 200 {
        vec = wash(&vaporized, &vec);
        let mut asteroids = visible_asteroids(&vec, pos.0, pos.1);
        asteroids.sort_by(|a, b| {
            let va = (a.0 as isize - pos.0 as isize, a.1 as isize - pos.1 as isize);
            let vb = (b.0 as isize - pos.0 as isize, b.1 as isize - pos.1 as isize);
            return angle(v0, va).partial_cmp(&angle(v0, vb)).unwrap();
        });

        vaporized.append(&mut asteroids);
    }

    vaporized[199].0 as usize * 100 + vaporized[199].1 as usize
}

fn asteroid_map(input: &Vec<String>) -> Vec<Vec<usize>> {
    let mut asteroid_map: Vec<Vec<usize>> = Vec::new();

    for x in 0..input.len() {
        let mut vec: Vec<usize> = Vec::new();
        for y in 0..input.len() {
            let asteroids = visible_asteroids(input, x, y);
            vec.push(asteroids.len());
        }
        asteroid_map.push(vec)
    }

    asteroid_map
}

fn starting_position(vec: &Vec<Vec<usize>>) -> (usize, usize) {
    let mut max: usize = 0;
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    let mut i: usize = 0;
    let mut j;

    for x in vec {
        j = 0;
        for y in x {
            if y > &max {
                max = *y;
                max_x = i;
                max_y = j;
            }
            j += 1;
        }
        i += 1;
    }
    (max_x, max_y)
}

fn visible_asteroids(vec: &Vec<String>, from_x: usize, from_y: usize) -> Vec<(f64, f64)> {
    let mut visible_asteroids = Vec::new();
    if !is_asteroid(vec, from_x, from_y) {
        return visible_asteroids;
    }

    for x in 0..vec.len() {
        for y in 0..vec.len() {
            let hit = line_of_sight(from_x, from_y, x, y)
                .filter(|(x, y)| x.fract() == 0.0 && y.fract() == 0.0)
                .find(|(x, y)| is_asteroid(&vec, *x as usize, *y as usize));

            if let Some(pos) = hit {
                if !visible_asteroids.contains(&pos) {
                    visible_asteroids.push(pos);
                }
            }
        }
    }

    visible_asteroids
}

fn line_of_sight(
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
) -> impl Iterator<Item = (f64, f64)> {
    let dx = to_x as isize - from_x as isize;
    let dy = to_y as isize - from_y as isize;

    let steps = std::cmp::max(dx.abs(), dy.abs());

    let step_x = dx as f64 / steps as f64;
    let step_y = dy as f64 / steps as f64;

    (1..=steps).map(move |step| {
        (
            from_x as f64 + step_x * step as f64,
            from_y as f64 + step_y * step as f64,
        )
    })
}

fn is_asteroid(vec: &Vec<String>, x: usize, y: usize) -> bool {
    vec[y].chars().nth(x).unwrap() == '#'
}

fn angle(v0: (isize, isize), v1: (isize, isize)) -> f64 {
    let angle = (dot(v0, v1) as f64 / (len(v0) * len(v1))).acos();
    if v1.0 < v0.0 {
        2.0 * std::f64::consts::PI - angle
    } else {
        angle
    }
}

fn dot(v0: (isize, isize), v1: (isize, isize)) -> isize {
    v0.0 * v1.0 + v0.1 * v1.1
}

fn len(v: (isize, isize)) -> f64 {
    ((v.0.pow(2) + v.1.pow(2)) as f64).sqrt()
}

fn wash(vaporized: &Vec<(f64, f64)>, vec: &Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    let mut i = 0;
    let mut j;
    for s in vec {
        i += 1;
        j = 0;
        let mut str = String::from("");
        for c in s.chars() {
            j += 1;
            if vaporized.contains(&(i as f64, j as f64)) {
                str.push('.');
                continue;
            }
            str.push(c);
        }
        res.push(str);
    }

    res
}

fn parse(input: &str) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    for line in input.lines() {
        vec.push(line.to_string());
    }

    vec
}

mod tests {
    #[test]
    fn test() {
        let input = std::fs::read_to_string("input/data_test")
            .expect("Something went wrong reading the file");
        let parsed = super::parse(&input);

        assert_eq!(super::visible_asteroids(&parsed, 0, 0).len(), 0);
        assert_eq!(super::visible_asteroids(&parsed, 0, 1).len(), 0);
        assert_eq!(super::visible_asteroids(&parsed, 0, 2).len(), 6);
        assert_eq!(super::visible_asteroids(&parsed, 0, 3).len(), 0);
        assert_eq!(super::visible_asteroids(&parsed, 0, 4).len(), 0);

        assert_eq!(super::visible_asteroids(&parsed, 1, 0).len(), 7);
        assert_eq!(super::visible_asteroids(&parsed, 1, 1).len(), 0);
        assert_eq!(super::visible_asteroids(&parsed, 1, 2).len(), 7);
        assert_eq!(super::visible_asteroids(&parsed, 1, 3).len(), 0);
        assert_eq!(super::visible_asteroids(&parsed, 1, 4).len(), 0);

        assert_eq!(super::visible_asteroids(&parsed, 2, 0).len(), 0);
        assert_eq!(super::visible_asteroids(&parsed, 2, 1).len(), 0);
        assert_eq!(super::visible_asteroids(&parsed, 2, 2).len(), 7);
        assert_eq!(super::visible_asteroids(&parsed, 2, 3).len(), 0);
        assert_eq!(super::visible_asteroids(&parsed, 2, 4).len(), 0);

        assert_eq!(super::visible_asteroids(&parsed, 3, 0).len(), 0);
        assert_eq!(super::visible_asteroids(&parsed, 3, 1).len(), 0);
        assert_eq!(super::visible_asteroids(&parsed, 3, 2).len(), 7);
        assert_eq!(super::visible_asteroids(&parsed, 3, 3).len(), 0);
        assert_eq!(super::visible_asteroids(&parsed, 3, 4).len(), 8);

        assert_eq!(super::visible_asteroids(&parsed, 4, 0).len(), 7);
        assert_eq!(super::visible_asteroids(&parsed, 4, 1).len(), 0);
        assert_eq!(super::visible_asteroids(&parsed, 4, 2).len(), 5);
        assert_eq!(super::visible_asteroids(&parsed, 4, 3).len(), 7);
        assert_eq!(super::visible_asteroids(&parsed, 4, 4).len(), 7);

        assert_eq!(super::part_one(), 247);
        assert_eq!(super::part_two(), 1919);
    }
}
