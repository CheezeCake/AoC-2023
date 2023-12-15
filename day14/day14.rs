use std::collections::HashMap;
use std::io;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum Shape {
    Cube,
    RoundedRock,
    Empty,
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Platform {
    map: Vec<Vec<Shape>>,
}

impl Platform {
    fn support_beams_load(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter().filter(|&s| *s == Shape::RoundedRock).count() * (self.map.len() - y)
            })
            .sum()
    }

    fn rotate(&mut self) {
        let n = self.map.len();

        for y in 0..n / 2 {
            for x in y..n - y - 1 {
                let tmp = self.map[y][x];
                self.map[y][x] = self.map[n - x - 1][y];
                self.map[n - x - 1][y] = self.map[n - y - 1][n - x - 1];
                self.map[n - y - 1][n - x - 1] = self.map[x][n - y - 1];
                self.map[x][n - y - 1] = tmp;
            }
        }
    }

    fn tilt(&mut self) {
        let n = self.map.len();
        // north
        for x in 0..n {
            let mut edge = 0;
            for y in 0..n {
                match self.map[y][x] {
                    Shape::Cube => edge = y + 1,
                    Shape::RoundedRock => {
                        self.map[y][x] = Shape::Empty;
                        self.map[edge][x] = Shape::RoundedRock;
                        edge += 1;
                    }
                    Shape::Empty => {}
                }
            }
        }
    }

    fn cycle(&mut self) {
        for _ in 0..4 {
            self.tilt();
            self.rotate();
        }
    }
}

fn main() {
    let input: Vec<Vec<Shape>> = io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    '#' => Shape::Cube,
                    'O' => Shape::RoundedRock,
                    '.' => Shape::Empty,
                    _ => panic!("invalid character: {}", c),
                })
                .collect()
        })
        .collect();

    let mut p = Platform { map: input };

    {
        let mut p = p.clone();
        p.tilt();
        println!("part 1: {}", p.support_beams_load());
    }

    let mut seen = HashMap::new();
    for i in 0.. {
        p.cycle();

        if let Some(j) = seen.get(&p) {
            let cycle_len = i - j;
            let cycles_left = (1_000_000_000 - j) % cycle_len - 1;
            for _ in 0..cycles_left {
                p.cycle();
            }
            println!("part 2: {}", p.support_beams_load());
            break;
        } else {
            seen.insert(p.clone(), i);
        }
    }
}
