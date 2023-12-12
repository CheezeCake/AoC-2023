use std::collections::HashMap;
use std::io;

enum Pattern {
    ZeroOrN(Box<dyn Fn(char) -> bool>),
    OneOrN(Box<dyn Fn(char) -> bool>),
    N(Box<dyn Fn(char) -> bool>, usize),
}

fn create_pattern(damaged_groups: &[usize]) -> Vec<Pattern> {
    let mut res = vec![Pattern::ZeroOrN(Box::new(|c| c == '.' || c == '?'))];

    for &group_size in damaged_groups {
        res.push(Pattern::N(Box::new(|c| c == '#' || c == '?'), group_size));
        res.push(Pattern::OneOrN(Box::new(|c| c == '.' || c == '?')));
    }
    res.pop();

    res.push(Pattern::ZeroOrN(Box::new(|c| c == '.' || c == '?')));

    res
}

fn match_count(
    rpos: usize,
    record: &[char],
    ppos: usize,
    pattern: &[Pattern],
    mem: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let k = (rpos, ppos);
    if let Some(&count) = mem.get(&k) {
        return count;
    }

    if rpos >= record.len() && ppos >= pattern.len() {
        mem.insert(k, 1);
        return 1;
    }
    if ppos >= pattern.len() {
        mem.insert(k, 0);
        return 0;
    }

    let count = match &pattern[ppos] {
        Pattern::ZeroOrN(pred) => {
            match_count(rpos, record, ppos + 1, pattern, mem)
                + if rpos < record.len() && pred(record[rpos]) {
                    match_count(rpos + 1, record, ppos, pattern, mem)
                } else {
                    0
                }
        }
        Pattern::OneOrN(pred) => {
            if rpos < record.len() && pred(record[rpos]) {
                match_count(rpos + 1, record, ppos + 1, pattern, mem)
                    + match_count(rpos + 1, record, ppos, pattern, mem)
            } else {
                0
            }
        }
        Pattern::N(pred, n) => {
            if rpos + n <= record.len() && record[rpos..rpos + n].iter().all(|&c| pred(c)) {
                match_count(rpos + n, record, ppos + 1, pattern, mem)
            } else {
                0
            }
        }
    };

    mem.insert(k, count);

    count
}

fn solve(data: &[(Vec<char>, Vec<usize>)]) -> usize {
    data.iter()
        .map(|(record, group_sizes)| {
            let mut mem = HashMap::new();
            match_count(0, record, 0, &create_pattern(group_sizes), &mut mem)
        })
        .sum::<usize>()
}

fn main() {
    let data: Vec<(Vec<char>, Vec<usize>)> = io::stdin()
        .lines()
        .map(|line| {
            let line = line.expect("error reading input");
            let (record, group_sizes) = line.split_once(' ').expect("error parsing input");
            let group_sizes = group_sizes
                .split(',')
                .map(|s| s.parse::<usize>().expect("error parsing group size"))
                .collect();
            (record.chars().collect(), group_sizes)
        })
        .collect();
    println!("part 1: {}", solve(&data));

    let unfolded_data: Vec<(Vec<char>, Vec<usize>)> = data
        .iter()
        .map(|(record, group_sizes)| {
            let mut unfolded_record = Vec::new();
            let mut unfolded_group_sizes = Vec::new();
            for _ in 0..5 {
                unfolded_record.append(&mut record.clone());
                unfolded_record.push('?');

                unfolded_group_sizes.append(&mut group_sizes.clone());
            }

            unfolded_record.pop();

            (unfolded_record, unfolded_group_sizes)
        })
        .collect();
    println!("part 2: {}", solve(&unfolded_data));
}
