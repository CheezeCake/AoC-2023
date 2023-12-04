use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

struct Number {
    y: usize,
    x: usize,
    len: usize,
    value: usize,
}

fn find_numbers(schematic: &[Vec<char>]) -> Vec<Number> {
    let mut numbers = Vec::new();

    for (y, line) in schematic.iter().enumerate() {
        let mut x: usize = 0;

        while x < line.len() {
            if line[x].is_digit(10) {
                let mut j = x;
                let mut value = 0;
                while j < line.len() && line[j].is_digit(10) {
                    value = value * 10 + line[j].to_digit(10).unwrap() as usize;
                    j += 1;
                }

                numbers.push(Number {
                    y,
                    x,
                    len: j - x,
                    value,
                });

                x = j;
            } else {
                x += 1;
            }
        }
    }

    numbers
}

fn adjacent_pos(
    y: usize,
    x: usize,
    schematic: &[Vec<char>],
) -> impl Iterator<Item = (usize, usize)> + '_ {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .filter_map(move |(dy, dx)| {
        let ny = y as isize + dy;
        let nx = x as isize + dx;
        if ny >= 0
            && (ny as usize) < schematic.len()
            && nx >= 0
            && (nx as usize) < schematic[ny as usize].len()
        {
            Some((ny as usize, nx as usize))
        } else {
            None
        }
    })
}

fn adjacent_chars(y: usize, x: usize, schematic: &[Vec<char>]) -> impl Iterator<Item = char> + '_ {
    adjacent_pos(y, x, schematic).map(move |(y, x)| schematic[y][x])
}

fn main() {
    let schematic: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let numbers = find_numbers(&schematic);
    let part_numbers: Vec<&Number> = numbers
        .iter()
        .filter(|number| {
            (number.x..number.x + number.len).any(|x| {
                adjacent_chars(number.y, x, &schematic).any(|c| c != '.' && !c.is_digit(10))
            })
        })
        .collect();
    println!(
        "part 1: {}",
        part_numbers
            .iter()
            .map(|number| number.value)
            .sum::<usize>()
    );

    let part_numbers_positions: HashMap<(usize, usize), usize> = part_numbers
        .iter()
        .enumerate()
        .map(|(i, number)| (number.x..number.x + number.len).map(move |x| ((number.y, x), i)))
        .flatten()
        .collect();
    let gear_ratios: Vec<usize> = schematic
        .iter()
        .map(|row| row.iter())
        .flatten()
        .enumerate()
        .filter(|(_, &c)| c == '*')
        .map(|(pos, _)| {
            let y = pos / schematic[0].len();
            let x = pos % schematic[0].len();
            adjacent_pos(y, x, &schematic)
                .filter_map(|adj_pos| part_numbers_positions.get(&adj_pos))
                .map(|index| *index)
                .collect::<HashSet<usize>>()
        })
        .filter(|set| set.len() == 2)
        .map(|set| set.iter().fold(1, |acc, &i| acc * part_numbers[i].value))
        .collect();
    println!("part 2: {}", gear_ratios.iter().sum::<usize>());
}
