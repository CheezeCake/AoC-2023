use std::collections::HashMap;
use std::io;

fn parse_seeds(s: &str) -> Vec<u64> {
    s.strip_prefix("seeds:")
        .expect("seeds not found")
        .split_whitespace()
        .filter_map(|n| n.parse::<u64>().ok())
        .collect::<Vec<u64>>()
}

fn read_maps() -> HashMap<String, Vec<MapRange>> {
    let mut maps: HashMap<String, Vec<MapRange>> = HashMap::new();
    let stdin = io::stdin();

    loop {
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Err(_) => panic!("error parsing input"),
            Ok(0) => break,
            Ok(1) => continue,
            Ok(_) => {
                let map_name = line[..line.find(' ').unwrap()].to_string();
                let mut map_ranges = Vec::new();

                loop {
                    let mut line = String::new();
                    match stdin.read_line(&mut line) {
                        Err(_) => panic!("error parsing input"),
                        Ok(0) | Ok(1) => break,
                        Ok(_) => {
                            let values: Vec<u64> = line
                                .split_whitespace()
                                .map(|s| s.parse::<u64>().unwrap())
                                .collect();
                            let dst = values[0];
                            let start = values[1];
                            let end = start + values[2] - 1;
                            map_ranges.push(MapRange {
                                r: Range { start, end },
                                dst,
                            });
                        }
                    }
                }

                map_ranges.sort_by_key(|mr| mr.r.start);
                maps.insert(map_name, map_ranges);
            }
        }
    }

    maps
}

#[derive(Copy, Clone)]
struct Range {
    start: u64,
    end: u64,
}

struct MapRange {
    r: Range,
    dst: u64,
}

fn map_ranges(srcs: &[Range], map: &[MapRange]) -> Vec<Range> {
    let mut dsts: Vec<Range> = srcs
        .iter()
        .map(|src| map_range(src, map).into_iter())
        .flatten()
        .collect();
    dsts.sort_by_key(|r| r.start);

    // merge ranges
    let mut idx = 0;
    for i in 1..dsts.len() {
        if dsts[i].start <= dsts[idx].end {
            dsts[idx].end = dsts[idx].end.max(dsts[i].end);
        } else {
            idx += 1;
            dsts[idx] = dsts[i];
        }
    }

    dsts[..=idx].to_vec()
}

fn map_range(src: &Range, map: &[MapRange]) -> Vec<Range> {
    let mut mapped = Vec::new();
    let mut idx = match map.binary_search_by_key(&src.start, |r| r.r.start) {
        Ok(i) => i,
        Err(i) => i,
    };
    if idx > 0 && src.start <= map[idx - 1].r.end {
        idx = idx - 1;
    }

    let mut cur_start = src.start;
    while cur_start <= src.end {
        let cur_end;

        if idx < map.len() && map[idx].r.start <= cur_start {
            cur_end = map[idx].r.end.min(src.end);
            let mapped_start = map[idx].dst + (cur_start - map[idx].r.start);
            let mapped_end = mapped_start + (cur_end - cur_start);
            mapped.push(Range {
                start: mapped_start,
                end: mapped_end,
            });
            idx += 1;
        } else {
            if idx < map.len() {
                cur_end = (map[idx].r.start - 1).min(src.end);
            } else {
                cur_end = src.end;
            }
            let mapped_start = cur_start;
            let mapped_end = cur_end;
            mapped.push(Range {
                start: mapped_start,
                end: mapped_end,
            });
        }

        cur_start = cur_end + 1;
    }

    mapped
}

fn lowest_location(seeds: &[Range], maps: &HashMap<String, Vec<MapRange>>) -> u64 {
    let map_order = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    let mut tmp = seeds.to_vec();

    for map_name in map_order {
        tmp = map_ranges(&tmp, maps.get(map_name).unwrap());
    }

    tmp.iter().map(|r| r.start).min().unwrap()
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let seeds = parse_seeds(&line);
    let maps = read_maps();

    let single_seed_ranges: Vec<Range> = seeds
        .iter()
        .map(|&seed| Range {
            start: seed,
            end: seed,
        })
        .collect();
    println!("part 1: {}", lowest_location(&single_seed_ranges, &maps));

    let seed_ranges: Vec<Range> = seeds
        .iter()
        .step_by(2)
        .zip(seeds.iter().skip(1).step_by(2))
        .map(|(&seed1, &seed2)| Range {
            start: seed1,
            end: seed1 + seed2 - 1,
        })
        .collect();
    println!("part 2: {}", lowest_location(&seed_ranges, &maps));
}
