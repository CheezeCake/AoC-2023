use std::io;

fn generate_sequences(initial_sequence: &[i64]) -> Vec<Vec<i64>> {
    let mut sequences = Vec::new();
    sequences.push(initial_sequence.to_vec());

    while sequences.last().unwrap().iter().any(|&x| x != 0) {
        let last_sequence = sequences.last().unwrap();
        let next_sequence: Vec<i64> = last_sequence
            .iter()
            .zip(last_sequence.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();
        sequences.push(next_sequence);
    }

    sequences
}

fn main() {
    let history: Vec<Vec<i64>> = io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(' ')
                .map(|s| s.parse::<i64>().expect("error parsing value"))
                .collect()
        })
        .collect();

    let sequences: Vec<Vec<Vec<i64>>> = history.iter().map(|seq| generate_sequences(seq)).collect();
    println!(
        "part 1: {}",
        sequences
            .iter()
            .map(|seqs| seqs.iter().map(|seq| *seq.last().unwrap()).sum::<i64>())
            .sum::<i64>()
    );
    println!(
        "part 2: {}",
        sequences
            .iter()
            .map(|seqs| seqs
                .iter()
                .rev()
                .map(|seq| seq[0])
                .reduce(|acc, x| x - acc)
                .unwrap())
            .sum::<i64>()
    );
}
