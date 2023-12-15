use std::collections::VecDeque;
use std::io;

fn hash(s: &str) -> u8 {
    s.bytes().fold(0, |acc, c| {
        let x = (acc as u32 + c as u32) * 17;
        (x % 256) as u8
    })
}

fn main() {
    let mut init_sequence = String::new();
    io::stdin().read_line(&mut init_sequence).unwrap();
    let steps: Vec<&str> = init_sequence.trim_end().split(',').collect();

    println!(
        "part 1: {}",
        steps.iter().map(|step| { hash(step) as u32 }).sum::<u32>()
    );

    let mut boxes: [VecDeque<(&str, usize)>; 256] = std::array::from_fn(|_| VecDeque::new());

    for step in steps {
        if let Some((label, focal_length)) = step.split_once('=') {
            let focal_length = focal_length.parse::<usize>().unwrap();
            let box_id = hash(label) as usize;
            let lens = boxes[box_id]
                .iter_mut()
                .enumerate()
                .find(|(_, (l, _))| *l == label);

            if let Some((_, (_, fl))) = lens {
                *fl = focal_length;
            } else {
                boxes[box_id].push_back((label, focal_length));
            }
        } else {
            let label = &step[0..step.len() - 1];
            let box_id = hash(label) as usize;
            let lens = boxes[box_id]
                .iter_mut()
                .enumerate()
                .find(|(_, (l, _))| *l == label);

            if let Some((i, _)) = lens {
                boxes[box_id].remove(i);
            }
        }
    }

    println!(
        "part 2: {}",
        boxes
            .iter()
            .enumerate()
            .map(|(box_id, b)| b
                .iter()
                .enumerate()
                .map(move |(slot, (_, focal_length))| (box_id + 1) * (slot + 1) * focal_length))
            .flatten()
            .sum::<usize>()
    );
}
