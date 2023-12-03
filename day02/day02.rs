use std::io;
use std::str::FromStr;

#[derive(Debug)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct ParseCubeSetError;

impl FromStr for CubeSet {
    type Err = ParseCubeSetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cs = CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        };
        let mut it = s.split(' ');

        while let Some(n_str) = it.next() {
            let n: usize = n_str.parse().or(Err(ParseCubeSetError))?;
            if let Some(color) = it.next() {
                match color.trim_end_matches(',') {
                    "red" => cs.red = n,
                    "green" => cs.green = n,
                    "blue" => cs.blue = n,
                    _ => return Err(ParseCubeSetError),
                }
            } else {
                return Err(ParseCubeSetError);
            }
        }

        Ok(cs)
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}

#[derive(Debug)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon = s.find(':').ok_or(ParseGameError)?;
        let id: usize = s[..colon]
            .strip_prefix("Game")
            .and_then(|s| s.trim().parse().ok())
            .ok_or(ParseGameError)?;
        let sets: Vec<CubeSet> = s[colon + 1..]
            .split(";")
            .map(|s| s.trim().parse().expect("error parsing set"))
            .collect();

        return Ok(Game { id, sets });
    }
}

fn main() {
    let games: Vec<Game> = io::stdin()
        .lines()
        .map(|line| line.unwrap().parse().expect("error parsing game"))
        .collect();

    println!(
        "part 1: {}",
        games
            .iter()
            .filter(|game| game
                .sets
                .iter()
                .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14))
            .map(|game| game.id)
            .sum::<usize>()
    );

    println!(
        "part 2: {}",
        games
            .into_iter()
            .map(|game| game
                .sets
                .into_iter()
                .reduce(|acc, e| CubeSet {
                    red: acc.red.max(e.red),
                    green: acc.green.max(e.green),
                    blue: acc.blue.max(e.blue),
                })
                .unwrap())
            .map(|min_set| min_set.red * min_set.green * min_set.blue)
            .sum::<usize>()
    );
}
