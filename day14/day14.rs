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
    map: Vec<Shape>,
    n: usize,
}

impl Platform {
    fn get(&self, x: usize, y: usize) -> Shape {
        self.map[y * self.n + x]
    }

    fn set(&mut self, x: usize, y: usize, s: Shape) {
        self.map[y * self.n + x] = s;
    }

    fn support_beams_load(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .filter(|(_, &s)| s == Shape::RoundedRock)
            .map(|(pos, _)| self.n - (pos / self.n))
            .sum()
    }

    fn rotate(&mut self) {
        for y in 0..self.n / 2 {
            for x in y..self.n - y - 1 {
                let tmp = self.get(x, y);
                self.set(x, y, self.get(y, self.n - x - 1));
                self.set(y, self.n - x - 1, self.get(self.n - x - 1, self.n - y - 1));
                self.set(self.n - x - 1, self.n - y - 1, self.get(self.n - y - 1, x));
                self.set(self.n - y - 1, x, tmp);
            }
        }
    }

    fn tilt(&mut self) {
        // north
        for x in 0..self.n {
            let mut edge = 0;
            for y in 0..self.n {
                match self.get(x, y) {
                    Shape::Cube => edge = y + 1,
                    Shape::RoundedRock => {
                        self.set(x, y, Shape::Empty);
                        self.set(x, edge, Shape::RoundedRock);
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
    let platform: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut p = Platform {
        map: platform
            .iter()
            .flatten()
            .map(|c| match c {
                '#' => Shape::Cube,
                'O' => Shape::RoundedRock,
                '.' => Shape::Empty,
                _ => panic!("invalid character: {}", c),
            })
            .collect(),
        n: platform.len(),
    };

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
