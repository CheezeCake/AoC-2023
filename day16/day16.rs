use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;
use std::io;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
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
}

fn within_bounds(p: &Point, cave: &[Vec<char>]) -> bool {
    p.y >= 0 && (p.y as usize) < cave.len() && p.x >= 0 && (p.x as usize) < cave[p.y as usize].len()
}

fn energized(dir: Direction, pos: &Point, cave: &[Vec<char>]) -> usize {
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((dir, *pos));

    while let Some((dir, pos)) = q.pop_front() {
        if !within_bounds(&pos, &cave) {
            continue;
        }
        if visited.contains(&(dir, pos)) {
            continue;
        }
        visited.insert((dir, pos));

        match cave[pos.y as usize][pos.x as usize] {
            '.' => q.push_back((dir, dir.next_pos(&pos))),
            '/' => match dir {
                Direction::Up => q.push_back((Direction::Right, Direction::Right.next_pos(&pos))),
                Direction::Down => q.push_back((Direction::Left, Direction::Left.next_pos(&pos))),
                Direction::Left => q.push_back((Direction::Down, Direction::Down.next_pos(&pos))),
                Direction::Right => q.push_back((Direction::Up, Direction::Up.next_pos(&pos))),
            },
            '\\' => match dir {
                Direction::Up => q.push_back((Direction::Left, Direction::Left.next_pos(&pos))),
                Direction::Down => q.push_back((Direction::Right, Direction::Right.next_pos(&pos))),
                Direction::Left => q.push_back((Direction::Up, Direction::Up.next_pos(&pos))),
                Direction::Right => q.push_back((Direction::Down, Direction::Down.next_pos(&pos))),
            },
            '|' => {
                q.push_back((Direction::Up, Direction::Up.next_pos(&pos)));
                q.push_back((Direction::Down, Direction::Down.next_pos(&pos)));
            }
            '-' => {
                q.push_back((Direction::Left, Direction::Left.next_pos(&pos)));
                q.push_back((Direction::Right, Direction::Right.next_pos(&pos)));
            }
            c => panic!("invalid character: {}", c),
        }
    }

    visited
        .iter()
        .map(|(_, pos)| pos)
        .collect::<HashSet<_>>()
        .len()
}

fn main() {
    let cave: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let (dir, pos) = (Direction::Right, Point { x: 0, y: 0 });
    println!("part 1: {}", energized(dir, &pos, &cave));

    let max_energized = (0..cave.len())
        .map(|i| {
            energized(
                Direction::Down,
                &Point {
                    x: i as isize,
                    y: 0,
                },
                &cave,
            )
            .max(energized(
                Direction::Up,
                &Point {
                    x: i as isize,
                    y: (cave.len() - 1) as isize,
                },
                &cave,
            ))
            .max(energized(
                Direction::Right,
                &Point {
                    x: 0,
                    y: i as isize,
                },
                &cave,
            ))
            .max(energized(
                Direction::Left,
                &Point {
                    x: (cave[0].len() - 1) as isize,
                    y: i as isize,
                },
                &cave,
            ))
        })
        .max()
        .unwrap();
    println!("part 2: {}", max_energized);
}
