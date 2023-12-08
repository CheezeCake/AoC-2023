use std::collections::HashMap;
use std::io;

fn steps_until(
    start: &String,
    instructions: &String,
    nodes: &HashMap<String, (String, String)>,
    pred: impl Fn(&String) -> bool,
) -> usize {
    let mut current_node = start;

    for (steps, instruction) in instructions.trim_end().chars().cycle().enumerate() {
        if pred(current_node) {
            return steps;
        }
        let (l, r) = nodes
            .get(current_node)
            .expect(format!("invalid node: {}", current_node).as_str());
        current_node = match instruction {
            'L' => l,
            'R' => r,
            _ => panic!("invalid instruction: {}", instruction),
        };
    }

    unreachable!()
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn main() {
    let mut instructions = String::new();
    io::stdin().read_line(&mut instructions).unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let nodes: HashMap<String, (String, String)> = io::stdin()
        .lines()
        .map(|line| {
            let line = line.expect("error reading input");
            let (node, lr) = line.split_once(" = ").expect("error parsing input");
            let (l, r) = lr
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .and_then(|s| s.split_once(", "))
                .expect("error parsing input");
            (node.to_string(), (l.to_string(), r.to_string()))
        })
        .collect();

    let start = String::from("AAA");
    println!(
        "part 1: {}",
        steps_until(&start, &instructions, &nodes, |node| node == "ZZZ")
    );

    println!(
        "part 2: {}",
        nodes
            .keys()
            .filter(|node| node.ends_with('A'))
            .map(|node| steps_until(node, &instructions, &nodes, |node| node.ends_with('Z')))
            .reduce(|acc, x| (acc * x) / gcd(acc, x))
            .unwrap()
    );
}
