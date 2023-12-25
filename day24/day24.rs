use std::io;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct ParseVec3Error;

impl FromStr for Vec3 {
    type Err = ParseVec3Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, yz) = s.split_once(',').ok_or(ParseVec3Error)?;
        let (y, z) = yz.split_once(',').ok_or(ParseVec3Error)?;
        Ok(Vec3 {
            x: x.trim().parse().map_err(|_| ParseVec3Error)?,
            y: y.trim().parse().map_err(|_| ParseVec3Error)?,
            z: z.trim().parse().map_err(|_| ParseVec3Error)?,
        })
    }
}

type Point = Vec3;
type Velocity = Vec3;

impl Point {
    fn advance(&self, t: i64, velocity: &Velocity) -> Self {
        Point {
            x: self.x + (t * velocity.x),
            y: self.y + (t * velocity.y),
            z: self.z + (t * velocity.z),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Hailstone {
    position: Point,
    velocity: Velocity,
}

impl Hailstone {
    fn can_reach(&self, x: f64) -> bool {
        (x >= self.position.x as f64 && self.velocity.x >= 0)
            || (x <= self.position.x as f64 && self.velocity.x <= 0)
    }
}

#[derive(Debug)]
struct ParseHailstoneError;

impl FromStr for Hailstone {
    type Err = ParseHailstoneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once('@').ok_or(ParseHailstoneError)?;
        Ok(Hailstone {
            position: p.trim().parse().map_err(|_| ParseHailstoneError)?,
            velocity: v.trim().parse().map_err(|_| ParseHailstoneError)?,
        })
    }
}

fn line_eq(hailstone: &Hailstone) -> (f64, f64) {
    let p1 = hailstone.position;
    let p2 = hailstone.position.advance(1, &hailstone.velocity);

    // y = ax + b
    let slope = (p2.y - p1.y) as f64 / (p2.x - p1.x) as f64;
    let b = p1.y as f64 - slope * p1.x as f64;

    (slope, b)
}

fn within_bounds(a: f64, min: f64, max: f64) -> bool {
    min <= a && a <= max
}

fn main() {
    let hailstones: Vec<Hailstone> = io::stdin()
        .lines()
        .map(|line| line.unwrap().parse().expect("error parsing hailstone"))
        .collect();

    // let (min_coord, max_coord) = (7f64, 27f64);
    let (min_coord, max_coord) = (200000000000000f64, 400000000000000f64);

    let intersections = hailstones
        .iter()
        .enumerate()
        .map(|(i, h1)| {
            hailstones.iter().skip(i + 1).filter(move |h2| {
                let (a1, b1) = line_eq(h1);
                let (a2, b2) = line_eq(h2);

                let x = (b2 - b1) / (a1 - a2);
                let y = a1 * x + b1;

                h1.can_reach(x)
                    && h2.can_reach(x)
                    && within_bounds(x, min_coord, max_coord)
                    && within_bounds(y, min_coord, max_coord)
            })
        })
        .flatten()
        .count();

    println!("part 1: {}", intersections);
}
