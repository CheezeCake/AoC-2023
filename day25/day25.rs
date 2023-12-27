use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

fn outward_edges(
    node: &String,
    group: &HashSet<&String>,
    graph: &HashMap<String, HashSet<String>>,
) -> usize {
    graph
        .get(node)
        .unwrap()
        .iter()
        .filter(|adj| !group.contains(adj))
        .count()
}

fn total_outward_edges(
    group: &HashSet<&String>,
    graph: &HashMap<String, HashSet<String>>,
) -> usize {
    group
        .iter()
        .map(|node| outward_edges(node, group, graph))
        .sum()
}

fn main() {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

    for line in io::stdin().lines() {
        let line = line.expect("error reading input");
        let (node, adj_list) = line.split_once(": ").expect("error parsing input");
        let mut adj_list: HashSet<_> = adj_list.split(' ').map(|s| s.to_string()).collect();

        for adj in &adj_list {
            graph
                .entry(node.to_string())
                .or_insert(HashSet::new())
                .insert(adj.to_string());
            graph
                .entry(adj.to_string())
                .or_insert(HashSet::new())
                .insert(node.to_string());
        }
    }

    let mut group: HashSet<_> = graph.keys().collect();
    while total_outward_edges(&group, &graph) != 3 {
        let node = group
            .iter()
            .map(|&node| (node, outward_edges(node, &group, &graph)))
            .max_by_key(|(_, n)| *n)
            .unwrap()
            .0
            .clone();
        group.remove(&node);
    }

    println!("part 1: {}", group.len() * (graph.len() - group.len()))
}
