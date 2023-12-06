use std::io;

fn read_input(p: &str) -> Vec<u64> {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("error reading input");

    line.strip_prefix(&(p.to_owned() + ":"))
        .expect("error parsing input")
        .split_whitespace()
        .filter_map(|n| n.parse::<u64>().ok())
        .collect()
}

fn ways_to_beat_record(time: u64, record: u64) -> usize {
    let delta = (time * time) - (4 * record);
    let sqrt_delta = (delta as f64).sqrt();
    let mut x1 = ((-(time as f64) - sqrt_delta) / -2 as f64).floor() as u64;
    let mut x2 = ((-(time as f64) + sqrt_delta) / -2 as f64).ceil() as u64;
    if x1 * (time - x1) <= record {
        x1 = x1 - 1;
    }
    if x2 * (time - x2) <= record {
        x2 = x2 + 1;
    }

    (x1 - x2 + 1) as usize
}

fn apply_kerning(data: &[u64]) -> u64 {
    data.iter()
        .map(|n| n.to_string())
        .reduce(|acc, n| acc + &n)
        .unwrap()
        .parse::<u64>()
        .unwrap()
}

fn main() {
    let times = read_input("Time");
    let distances = read_input("Distance");
    println!(
        "part 1: {}",
        times
            .iter()
            .zip(&distances)
            .map(|(&time, &distance)| ways_to_beat_record(time, distance))
            .reduce(|acc, n| acc * n)
            .unwrap()
    );

    let time = apply_kerning(&times);
    let distance = apply_kerning(&distances);
    println!("part 2: {}", ways_to_beat_record(time, distance));
}
