use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next_pos(&self, pos: &Point) -> Point {
        match self {
            Self::Up => Point {
                x: pos.x,
                y: pos.y - 1,
            },
            Self::Down => Point {
                x: pos.x,
                y: pos.y + 1,
            },
            Self::Left => Point {
                x: pos.x - 1,
                y: pos.y,
            },
            Self::Right => Point {
                x: pos.x + 1,
                y: pos.y,
            },
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

fn within_bounds(p: &Point, map: &[Vec<u32>]) -> bool {
    p.y >= 0 && (p.y as usize) < map.len() && p.x >= 0 && (p.x as usize) < map[p.y as usize].len()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Crucible {
    pos: Point,
    dir: Direction,
    dir_count: usize,
    max_dir_count: usize,
    min_dir_count: usize,
}

impl Crucible {
    fn new_standard(pos: Point, dir: Direction) -> Self {
        Self {
            pos,
            dir,
            dir_count: 1,
            max_dir_count: 3,
            min_dir_count: 0,
        }
    }

    fn new_ultra(pos: Point, dir: Direction) -> Self {
        Self {
            pos,
            dir,
            dir_count: 1,
            max_dir_count: 10,
            min_dir_count: 4,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    heat_loss: u32,
    crucible: Crucible,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| self.crucible.cmp(&other.crucible))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(crucible: &Crucible, goal: &Point, map: &[Vec<u32>]) -> Option<u32> {
    let mut losses: HashMap<Crucible, u32> = HashMap::new();

    let mut pq = BinaryHeap::new();

    losses.insert(*crucible, 0);
    pq.push(State {
        heat_loss: 0,
        crucible: *crucible,
    });

    while let Some(State {
        heat_loss,
        crucible,
    }) = pq.pop()
    {
        let Crucible {
            pos,
            dir,
            dir_count,
            max_dir_count,
            min_dir_count,
        } = crucible;

        if pos == *goal {
            continue;
        }

        if heat_loss > *losses.get(&crucible).unwrap_or(&u32::MAX) {
            continue;
        }

        for next_dir in [dir, dir.turn_left(), dir.turn_right()] {
            let next_dir_count = if next_dir == dir { dir_count + 1 } else { 1 };
            if next_dir_count > max_dir_count {
                continue;
            }
            if next_dir != dir && dir_count < min_dir_count {
                continue;
            }
            let next_pos = next_dir.next_pos(&pos);
            if !within_bounds(&next_pos, map) {
                continue;
            }

            let next = State {
                heat_loss: heat_loss + map[next_pos.y as usize][next_pos.x as usize],
                crucible: Crucible {
                    pos: next_pos,
                    dir: next_dir,
                    dir_count: next_dir_count,
                    max_dir_count: max_dir_count,
                    min_dir_count: min_dir_count,
                },
            };

            if next.heat_loss < *losses.get(&next.crucible).unwrap_or(&u32::MAX) {
                pq.push(next);
                losses.insert(next.crucible, next.heat_loss);
            }
        }
    }

    losses
        .iter()
        .filter(|(crucible, _)| {
            crucible.pos == *goal
                && crucible.dir_count >= crucible.min_dir_count
                && crucible.dir_count <= crucible.max_dir_count
        })
        .map(|(_, &heat_loss)| heat_loss)
        .min()
}

fn main() {
    let map: Vec<Vec<u32>> = io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).expect("map character is not a digit"))
                .collect()
        })
        .collect();

    let goal = Point {
        x: (map[0].len() - 1) as isize,
        y: (map.len() - 1) as isize,
    };

    let crucible = Crucible::new_standard(Point { x: 0, y: 0 }, Direction::Right);
    println!(
        "part 1: {}",
        solve(&crucible, &goal, &map).expect("no solution for part 1")
    );

    let crucible = Crucible::new_ultra(Point { x: 0, y: 0 }, Direction::Right);
    println!(
        "part 2: {}",
        solve(&crucible, &goal, &map).expect("no solution for part 2")
    );
}
