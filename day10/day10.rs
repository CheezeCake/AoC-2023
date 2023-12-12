use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

fn find_start(grid: &[Vec<char>]) -> Option<Point> {
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                return Some(Point {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }
    None
}

fn within_bounds(tile: &Point, grid: &[Vec<char>]) -> bool {
    tile.y >= 0
        && (tile.y as usize) < grid.len()
        && tile.x >= 0
        && (tile.x as usize) < grid[tile.y as usize].len()
}

fn next_tile(tile: &Point, dx: isize, dy: isize, grid: &[Vec<char>]) -> Option<Point> {
    let x = (tile.x as isize) + dx;
    let y = (tile.y as isize) + dy;
    if within_bounds(&Point { x, y }, grid) {
        Some(Point { x, y })
    } else {
        None
    }
}

fn connected_pipes(pipe: &Point, grid: &[Vec<char>]) -> (Option<Point>, Option<Point>) {
    match grid[pipe.y as usize][pipe.x as usize] {
        '|' => (next_tile(pipe, 0, -1, grid), next_tile(pipe, 0, 1, grid)),
        '-' => (next_tile(pipe, -1, 0, grid), next_tile(pipe, 1, 0, grid)),
        'L' => (next_tile(pipe, 0, -1, grid), next_tile(pipe, 1, 0, grid)),
        'J' => (next_tile(pipe, 0, -1, grid), next_tile(pipe, -1, 0, grid)),
        '7' => (next_tile(pipe, 0, 1, grid), next_tile(pipe, -1, 0, grid)),
        'F' => (next_tile(pipe, 0, 1, grid), next_tile(pipe, 1, 0, grid)),
        _ => (None, None),
    }
}

fn next_tile_expect(
    p: &Point,
    dx: isize,
    dy: isize,
    grid: &[Vec<char>],
    pat: &str,
) -> Option<Point> {
    if let Some(next) = next_tile(p, dx, dy, grid) {
        if pat.find(grid[next.y as usize][next.x as usize]).is_some() {
            Some(next)
        } else {
            None
        }
    } else {
        None
    }
}

fn start_pipe(start: &Point, grid: &[Vec<char>]) -> char {
    let north = next_tile_expect(&start, 0, -1, grid, "|7F");
    let south = next_tile_expect(&start, 0, 1, grid, "|JL");
    let east = next_tile_expect(&start, -1, 0, grid, "-LF");
    let west = next_tile_expect(&start, 1, 0, grid, "-J7");

    match (north, south, east, west) {
        (Some(_), Some(_), None, None) => '|',
        (Some(_), None, Some(_), None) => 'J',
        (Some(_), None, None, Some(_)) => 'L',
        (None, Some(_), Some(_), None) => '7',
        (None, Some(_), None, Some(_)) => 'F',
        (None, None, Some(_), Some(_)) => '-',
        _ => unreachable!(),
    }
}

fn main() {
    let mut grid: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let start = find_start(&grid).unwrap();
    grid[start.y as usize][start.x as usize] = start_pipe(&start, &grid);

    let mut loop_pipes_distances: HashMap<Point, usize> = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0));

    while let Some((p, distance)) = q.pop_front() {
        if loop_pipes_distances.contains_key(&p) {
            continue;
        }
        loop_pipes_distances.insert(p, distance);

        let (next1, next2) = connected_pipes(&p, &grid);
        if let Some(next) = next1 {
            q.push_back((next, distance + 1));
        }
        if let Some(next) = next2 {
            q.push_back((next, distance + 1));
        }
    }

    println!("part 1: {}", loop_pipes_distances.values().max().unwrap());

    let mut inside = false;
    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let p = Point {
                x: x as isize,
                y: y as isize,
            };
            if loop_pipes_distances.contains_key(&p) {
                let c = grid[y as usize][x as usize];
                if c == '|' || c == 'J' || c == 'L' {
                    inside = !inside;
                }
                continue;
            }

            if inside {
                count += 1;
            }
        }
    }

    println!("part 2: {}", count);
}
