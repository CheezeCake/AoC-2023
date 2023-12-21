use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn next_pos(&self, pos: &Point) -> Point {
        match self {
            Self::North => Point {
                x: pos.x,
                y: pos.y - 1,
            },
            Self::South => Point {
                x: pos.x,
                y: pos.y + 1,
            },
            Self::West => Point {
                x: pos.x - 1,
                y: pos.y,
            },
            Self::East => Point {
                x: pos.x + 1,
                y: pos.y,
            },
        }
    }
}

fn find_start(map: &[Vec<char>]) -> Option<Point> {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                return Some(Point {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }
    None
}

fn main() {
    let map: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let start = find_start(&map).unwrap();

    let mut q = VecDeque::new();
    q.push_back((start, 0));

    let mut visited: HashMap<Point, usize> = HashMap::new();

    let max_steps = 64;

    while let Some((p, steps)) = q.pop_front() {
        // let y_len = map.len() as isize;
        // let y = if p.y < 0 {
        //     (y_len + (p.y % y_len) - 1) as usize
        // } else {
        //     (p.y % y_len) as usize
        // };
        // let x_len = map[y].len() as isize;
        // let x = if p.x < 0 {
        //     (x_len + (p.x % x_len) - 1) as usize
        // } else {
        //     (p.x % x_len) as usize
        // };
        // if map[y][x] == '#' {
        if map[p.y as usize][p.x as usize] == '#' {
            continue;
        }

        if visited.contains_key(&p) {
            continue;
        }
        visited.insert(p, steps);

        if steps == max_steps {
            continue;
        }

        for dir in [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ] {
            let next_pos = dir.next_pos(&p);
            q.push_back((next_pos, steps + 1));
        }
    }

    println!(
        "part 1: {}",
        visited.iter().filter(|(_, &steps)| steps % 2 == 0).count()
    );
}
