use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn next(&self, direction: &Point) -> Point {
        Point {
            x: self.x + direction.x,
            y: self.y + direction.y,
        }
    }
}

const UP: Point = Point { x: 0, y: -1 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };

fn within_bounds(p: &Point, map: &[Vec<char>]) -> bool {
    p.y >= 0 && (p.y as usize) < map.len() && p.x >= 0 && (p.x as usize) < map[p.y as usize].len()
}

fn neighbours(pos: &Point, map: &[Vec<char>]) -> Vec<Point> {
    [UP, DOWN, LEFT, RIGHT]
        .iter()
        .map(|dir| pos.next(dir))
        .filter(|adj| within_bounds(&adj, map) && map[adj.y as usize][adj.x as usize] != '#')
        .collect()
}

fn dfs(
    pos: &Point,
    goal: &Point,
    map: &[Vec<char>],
    visited: &mut HashSet<Point>,
) -> Option<usize> {
    if pos == goal {
        return Some(visited.len());
    }

    if visited.contains(pos) {
        return None;
    }
    visited.insert(*pos);

    let n = match map[pos.y as usize][pos.x as usize] {
        '^' => dfs(&pos.next(&UP), goal, map, visited),
        'v' => dfs(&pos.next(&DOWN), goal, map, visited),
        '<' => dfs(&pos.next(&LEFT), goal, map, visited),
        '>' => dfs(&pos.next(&RIGHT), goal, map, visited),
        _ => neighbours(pos, map)
            .iter()
            .filter_map(|p| dfs(p, goal, map, visited))
            .max(),
    };

    visited.remove(pos);

    n
}

fn dfs2(
    pos: &Point,
    distance: usize,
    goal: &Point,
    graph: &HashMap<Point, Vec<(Point, usize)>>,
    visited: &mut HashSet<Point>,
) -> Option<usize> {
    if pos == goal {
        return Some(distance);
    }

    if visited.contains(pos) {
        return None;
    }
    visited.insert(*pos);

    let n = graph
        .get(pos)
        .unwrap()
        .iter()
        .filter_map(|(node, distance)| dfs2(node, *distance, goal, graph, visited))
        .max();

    visited.remove(pos);

    n.and_then(|n| Some(distance + n))
}

fn contract(
    u: &Point,
    start: &Point,
    goal: &Point,
    map: &[Vec<char>],
    graph: &mut HashMap<Point, Vec<(Point, usize)>>,
) {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    visited.insert(*u);
    graph.insert(*u, Vec::new());

    for neighbour in neighbours(u, map) {
        q.push_back((neighbour, 1));
    }

    while let Some((p, distance)) = q.pop_front() {
        if visited.contains(&p) {
            continue;
        }
        visited.insert(p);

        let nodes = neighbours(&p, map);
        if p == *start || p == *goal || nodes.len() > 2 {
            graph.get_mut(u).unwrap().push((p, distance));
        } else {
            for node in nodes {
                q.push_back((node, distance + 1));
            }
        }
    }
}

fn contracted_graph(
    start: &Point,
    goal: &Point,
    map: &[Vec<char>],
) -> HashMap<Point, Vec<(Point, usize)>> {
    let junctions: Vec<Point> = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c != '#')
                .map(move |(x, _)| Point {
                    x: x as isize,
                    y: y as isize,
                })
                .filter(|p| neighbours(p, &map).len() > 2)
        })
        .flatten()
        .collect();

    let mut graph = HashMap::new();

    for junction in junctions {
        contract(&junction, &start, &goal, &map, &mut graph);
    }
    contract(&start, &start, &goal, &map, &mut graph);
    contract(&goal, &start, &goal, &map, &mut graph);

    graph
}

fn single_path_tile_index(row: &[char]) -> Option<usize> {
    row.iter()
        .enumerate()
        .filter(|(_, &c)| c != '#')
        .map(|(i, _)| i)
        .next()
}

fn main() {
    let map: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let start = Point {
        x: single_path_tile_index(&map[0]).unwrap() as isize,
        y: 0,
    };
    let goal = Point {
        x: single_path_tile_index(map.last().unwrap()).unwrap() as isize,
        y: (map.len() - 1) as isize,
    };

    println!(
        "part 1: {}",
        dfs(&start, &goal, &map, &mut HashSet::new()).unwrap()
    );

    let graph = contracted_graph(&start, &goal, &map);
    println!(
        "part 2: {}",
        dfs2(&start, 0, &goal, &graph, &mut HashSet::new()).unwrap()
    );
}
