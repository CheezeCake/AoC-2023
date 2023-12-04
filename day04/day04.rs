use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

fn main() {
    let cards_matching_numbers_count: HashMap<usize, usize> = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let colon = line.find(':').expect("error parsing card");
            let id = line[..colon]
                .strip_prefix("Card")
                .and_then(|s| s.trim().parse().ok())
                .expect("error parsing card");
            let matching_numbers_count = line[colon + 1..]
                .split('|')
                .map(|set_str| {
                    set_str
                        .split_whitespace()
                        .map(|n_str| n_str.parse::<u64>().unwrap())
                        .collect::<HashSet<u64>>()
                })
                .reduce(|acc, set| acc.intersection(&set).map(|v| *v).collect())
                .unwrap_or(HashSet::new())
                .len();

            (id, matching_numbers_count)
        })
        .collect();
    println!(
        "part 1: {}",
        cards_matching_numbers_count
            .iter()
            .filter(|(_, &count)| count > 0)
            .map(|(_, &count)| 2usize.pow(count as u32 - 1))
            .sum::<usize>()
    );

    let mut instances = vec![1; cards_matching_numbers_count.len()];
    for card_id in 0..cards_matching_numbers_count.len() {
        let matching_count = cards_matching_numbers_count.get(&(card_id + 1)).unwrap();
        for copy_id in card_id + 1..=card_id + matching_count {
            instances[copy_id] += instances[card_id];
        }
    }
    println!("part 2: {}", instances.iter().sum::<usize>());
}
