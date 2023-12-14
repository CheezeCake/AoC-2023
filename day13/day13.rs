use std::io;
use std::iter;

fn reflection_line(lines: &[u32], smudges: usize) -> Option<usize> {
    for i in 0..lines.len() - 1 {
        let mut j = i as isize;
        let mut k = i + 1;

        let mut n = 0;

        while j >= 0 && k < lines.len() {
            let x = lines[j as usize] ^ lines[k];
            if x != 0 {
                if x.count_ones() == 1 && n < smudges {
                    n += 1;
                } else {
                    break;
                }
            }

            j -= 1;
            k += 1;
        }

        if j < 0 || k >= lines.len() {
            if n == smudges {
                return Some(i + 1);
            }
        }
    }

    None
}

fn pattern_char_to_bit(c: u8) -> u32 {
    match c {
        b'.' => 0,
        b'#' => 1,
        _ => panic!("invalid character: {}", c),
    }
}

fn solve(patterns_lines: &[Vec<u32>], patterns_cols: &[Vec<u32>], smudges: usize) -> usize {
    let (horizontal, vertical) = iter::zip(patterns_lines, patterns_cols)
        .map(|(lines, cols)| {
            if let Some(reflection_line) = reflection_line(&lines, smudges) {
                (reflection_line, 0)
            } else {
                (0, reflection_line(&cols, smudges).unwrap_or(0))
            }
        })
        .reduce(|(acc_h, acc_v), (h, v)| (acc_h + h, acc_v + v))
        .unwrap();

    vertical + 100 * horizontal
}

fn main() {
    let input: Vec<Vec<u8>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().bytes().collect())
        .collect();
    let pattern_boundaries: Vec<usize> = input
        .iter()
        .enumerate()
        .filter(|(_, line)| line.len() == 0)
        .map(|(i, _)| i)
        .collect();
    let patterns: Vec<&[Vec<u8>]> = iter::once(&input[0..pattern_boundaries[0]])
        .chain(
            pattern_boundaries
                .iter()
                .zip(
                    pattern_boundaries
                        .iter()
                        .skip(1)
                        .chain(iter::once(&input.len())),
                )
                .map(|(&start, &end)| &input[start + 1..end]),
        )
        .collect();

    let patterns_lines: Vec<Vec<u32>> = patterns
        .iter()
        .map(|pattern| {
            pattern
                .iter()
                .map(|line| {
                    line.iter()
                        .map(|&c| pattern_char_to_bit(c))
                        .reduce(|acc, b| (acc << 1) | b)
                        .unwrap()
                })
                .collect()
        })
        .collect();
    let patterns_cols: Vec<Vec<u32>> = patterns
        .iter()
        .map(|pattern| {
            (0..pattern[0].len())
                .map(|x| {
                    (0..pattern.len())
                        .map(|y| pattern_char_to_bit(pattern[y][x]))
                        .reduce(|acc, b| (acc << 1) | b)
                        .unwrap()
                })
                .collect()
        })
        .collect();

    println!("part 1: {}", solve(&patterns_lines, &patterns_cols, 0));
    println!("part 2: {}", solve(&patterns_lines, &patterns_cols, 1));
}
