use std::collections::HashMap;
use std::io;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Debug)]
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

struct DigInstruction {
    direction: Direction,
    distance: usize,
    color: u32,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct Range {
    start: isize,
    end: isize,
}

fn intersection(r1: &Range, r2: &Range) -> Option<Range> {
    if r2.end > r1.start && r2.start < r1.end {
        Some(Range {
            start: r1.start.max(r2.start),
            end: r1.end.min(r2.end),
        })
    } else {
        None
    }
}

fn todo(done: &[Range], range: &Range) -> Option<Range> {
    if done.len() == 0 || range.end < done[0].start || range.start > done.last().unwrap().end {
        return Some(*range);
    }

    if done.len() == 1 {
        if range.start < done[0].start {
            return Some(Range {
                start: range.start,
                end: done[0].start,
            });
        } else if range.end > done[0].end {
            return Some(Range {
                start: done[0].end,
                end: range.end,
            });
        } else {
            return None;
        }
    }

    let mut i = 0;
    while i < done.len() && range.start >= done[i].start {
        i += 1;
    }

    if i == 0 {
        return Some(Range {
            start: range.start,
            end: done[i].start,
        });
    } else if i == done.len() {
        return Some(Range {
            start: done[i - 1].end,
            end: range.end,
        });
    } else {
        let start = if range.start <= done[i - 1].end {
            done[i - 1].end
        } else {
            range.start
        };
        let end = if range.end >= done[i].start {
            done[i].start
        } else {
            range.end
        };
        return Some(Range { start, end });
    }
}

fn range_done(range: &Range, done: &[Range]) -> bool {
    done.len() == 1 && done[0] == *range
}

fn add_done_range(done: &mut Vec<Range>, range: &Range) -> Option<(isize, isize)> {
    done.push(*range);
    done.sort();

    let mut merged = None;
    let mut idx = 0;
    for i in 1..done.len() {
        if done[i].start <= done[idx].end + 1 {
            merged = Some((
                done[idx].end.min(done[i].end),
                done[idx].end.max(done[i].end),
            ));
            done[idx].end = done[idx].end.max(done[i].end);
        } else {
            idx += 1;
            done[idx] = done[i];
        }
    }

    done.resize(idx + 1, Range { start: 0, end: 0 });

    merged
}

fn lagoon_capacity(dig_plan: &[DigInstruction]) -> usize {
    let mut v_ranges: Vec<(isize, Range)> = Vec::new();

    let start = Point { x: 0, y: 0 };

    let mut pos = start;
    for instruction in dig_plan {
        match instruction.direction {
            Direction::Up => {
                let start = pos.y - (instruction.distance as isize);
                let end = pos.y;
                pos = Point { x: pos.x, y: start };

                v_ranges.push((pos.x, Range { start, end }));
            }
            Direction::Down => {
                let start = pos.y;
                let end = pos.y + (instruction.distance as isize);
                pos = Point { x: pos.x, y: end };

                v_ranges.push((pos.x, Range { start, end }));
            }
            Direction::Left => {
                let start = pos.x - (instruction.distance as isize);
                // let end = pos.x;
                pos = Point { x: start, y: pos.y };
            }
            Direction::Right => {
                // let start = pos.x;
                let end = pos.x + (instruction.distance as isize);
                pos = Point { x: end, y: pos.y };
            }
        };
    }

    v_ranges.sort();

    let mut capacity = 0;

    let mut done: Vec<Vec<Range>> = v_ranges.iter().map(|_| Vec::new()).collect();
    let mut top_bottom: HashMap<isize, Vec<Range>> = HashMap::new();

    for i in 0..v_ranges.len() {
        let mut j = i + 1;
        while !range_done(&v_ranges[i].1, &done[i]) && j < v_ranges.len() {
            if let Some(inter) = intersection(&v_ranges[i].1, &v_ranges[j].1) {
                if let Some(inter) = todo(&done[i], &inter) {
                    if inter.start >= inter.end {
                        j += 1;
                        continue;
                    }
                    let h = inter.end - inter.start + 1;
                    let w = v_ranges[j].0 - v_ranges[i].0 + 1;
                    capacity += h * w;

                    add_done_range(&mut done[i], &inter);
                    add_done_range(&mut done[j], &inter);

                    let h_range = Range {
                        start: v_ranges[i].0,
                        end: v_ranges[j].0,
                    };
                    if let Some((inter_end, _)) = add_done_range(
                        top_bottom.entry(inter.start).or_insert(Vec::new()),
                        &h_range,
                    ) {
                        capacity -= inter_end - h_range.start + 1;
                    }
                    if let Some((inter_end, _)) =
                        add_done_range(top_bottom.entry(inter.end).or_insert(Vec::new()), &h_range)
                    {
                        capacity -= inter_end - h_range.start + 1;
                    }
                }
            }
            j += 1;
        }
        if !range_done(&v_ranges[i].1, &done[i]) {
            panic!("{}: {:?} , done[{}] = {:?}", i, v_ranges[i], i, done[i]);
        }
    }

    capacity as usize
}

fn main() {
    let dig_plan: Vec<DigInstruction> = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (direction, rest) = line.split_once(' ').expect("invalid input");
            let (distance, color) = rest.split_once(' ').expect("invalid input");
            let direction = match direction {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("invalid direction: {}", direction),
            };
            let distance: usize = distance.parse().expect("error parsing distance");
            let color = color
                .strip_prefix("(#")
                .and_then(|s| s.strip_suffix(')'))
                .expect("error parseing color");
            let color = u32::from_str_radix(color, 16).expect("error parsing color");

            DigInstruction {
                direction,
                distance,
                color,
            }
        })
        .collect();
    println!("part 1: {}", lagoon_capacity(&dig_plan));

    let dig_plan: Vec<DigInstruction> = dig_plan
        .iter()
        .map(|instruction| {
            let distance = (instruction.color >> 4) & 0xfffff;
            let direction = match instruction.color & 0xf {
                0 => Direction::Right,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Up,
                _ => panic!("invalid direction in hex code"),
            };
            DigInstruction {
                direction,
                distance: distance as usize,
                color: instruction.color,
            }
        })
        .collect();
    println!("part 2: {}", lagoon_capacity(&dig_plan));
}
