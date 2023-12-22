use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn next(&self, direction: &Self) -> Self {
        Self {
            x: self.x + direction.x,
            y: self.y + direction.y,
            z: self.z + direction.z,
        }
    }
}

#[derive(Debug)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, yz) = s.split_once(',').ok_or(ParsePointError)?;
        let (y, z) = yz.split_once(',').ok_or(ParsePointError)?;

        Ok(Point {
            x: x.parse().map_err(|_| ParsePointError)?,
            y: y.parse().map_err(|_| ParsePointError)?,
            z: z.parse().map_err(|_| ParsePointError)?,
        })
    }
}

#[derive(Copy, Clone, Debug)]
struct Brick {
    start: Point,
    end: Point,
}

impl Brick {
    fn direction(&self) -> Point {
        Point {
            x: if self.start.x != self.end.x { 1 } else { 0 },
            y: if self.start.y != self.end.y { 1 } else { 0 },
            z: if self.start.z != self.end.z { 1 } else { 0 },
        }
    }

    fn is_vertical(&self) -> bool {
        self.start.z != self.end.z
    }

    fn base(&self) -> (Point, Point) {
        if self.is_vertical() {
            (self.start, self.start)
        } else {
            (self.start, self.end)
        }
    }
}

#[derive(Debug)]
struct ParseBrickError;

impl FromStr for Brick {
    type Err = ParseBrickError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('~').ok_or(ParseBrickError)?;
        let brick = Brick {
            start: start.parse().map_err(|_| ParseBrickError)?,
            end: end.parse().map_err(|_| ParseBrickError)?,
        };
        if brick.start.x > brick.end.x || brick.start.y > brick.end.y || brick.start.z > brick.end.z
        {
            Ok(Brick {
                start: brick.end,
                end: brick.start,
            })
        } else {
            Ok(brick)
        }
    }
}

type State = HashMap<(usize, usize), Vec<(usize, usize)>>;

fn build_state(bricks: &[Brick]) -> State {
    let mut state: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for (i, brick) in bricks.iter().enumerate() {
        let mut pos = brick.start;
        let dir = brick.direction();

        loop {
            state
                .entry((pos.x, pos.y))
                .or_insert(Vec::new())
                .push((pos.z, i));
            if pos == brick.end {
                break;
            }
            pos = Point {
                x: pos.x + dir.x,
                y: pos.y + dir.y,
                z: pos.z + dir.z,
            };
        }
    }
    for zs in state.values_mut() {
        zs.sort();
    }

    state
}

fn make_bricks_fall(bricks: &mut [Brick]) {
    let mut done = false;

    while !done {
        let state = build_state(bricks);

        done = true;

        for brick in &mut *bricks {
            let (base_start, base_end) = brick.base();
            let mut pos = base_start;
            let dir = brick.direction();

            let mut max_z = 0;
            loop {
                let zs = state.get(&(pos.x, pos.y)).unwrap();
                let z_idx = zs.binary_search_by_key(&pos.z, |&(z, _)| z).unwrap();
                let prev_z = if z_idx == 0 {
                    0
                } else {
                    zs.get(z_idx - 1).unwrap_or(&(0, 0)).0
                };
                max_z = max_z.max(prev_z + 1);

                if pos == base_end {
                    break;
                }
                pos = pos.next(&dir);
            }

            let z_diff = pos.z - max_z;
            brick.start.z -= z_diff;
            brick.end.z -= z_diff;

            if z_diff > 0 {
                done = false;
            }
        }
    }
}

fn would_fall(
    brick: usize,
    supporting: &[HashSet<usize>],
    supported_by: &[HashSet<usize>],
) -> usize {
    let mut q = VecDeque::new();
    for &supported in &supporting[brick] {
        q.push_back(supported);
    }

    let mut fell = HashSet::new();
    fell.insert(brick);

    while let Some(brick) = q.pop_front() {
        if fell.contains(&brick) {
            continue;
        }

        if supported_by[brick].intersection(&fell).count() == supported_by[brick].len() {
            fell.insert(brick);
        }

        for &supported in &supporting[brick] {
            q.push_back(supported);
        }
    }

    fell.len()
}

fn main() {
    let mut bricks: Vec<Brick> = io::stdin()
        .lines()
        .map(|line| {
            line.expect("error reading input")
                .parse()
                .expect("error parsing brick")
        })
        .collect();

    make_bricks_fall(&mut bricks);

    let mut supported_by: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];
    let mut supporting: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];
    let state = build_state(&bricks);

    for (i, brick) in bricks.iter().enumerate() {
        let (base_start, base_end) = brick.base();
        let mut pos = base_start;
        let dir = brick.direction();

        loop {
            let zs = state.get(&(pos.x, pos.y)).unwrap();
            let z_idx = zs.binary_search_by_key(&pos.z, |&(z, _)| z).unwrap();
            let support = if z_idx == 0 {
                None
            } else {
                match zs.get(z_idx - 1) {
                    Some((z, i)) if z + 1 == pos.z => Some(i),
                    _ => None,
                }
            };
            if let Some(&support) = support {
                if support != i {
                    supported_by[i].insert(support);
                    supporting[support].insert(i);
                }
            }

            if pos == base_end {
                break;
            }
            pos = pos.next(&dir);
        }
    }

    println!(
        "part 1: {}",
        supporting
            .iter()
            .enumerate()
            .filter(|(_, supported)| supported
                .iter()
                .all(|&supported| supported_by[supported].len() > 1))
            .count()
    );

    println!(
        "part 2: {}",
        (0..bricks.len())
            .map(|i| would_fall(i, &supporting, &supported_by))
            .map(|n| n - 1)
            .sum::<usize>()
    );
}
