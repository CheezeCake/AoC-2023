use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

#[derive(Debug)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcast,
}

#[derive(Debug)]
struct Module {
    modtype: ModuleType,
    destinations: Vec<String>,
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

fn find_cycles(
    rx_input_inputs: &[String],
    modules: &mut HashMap<String, Module>,
) -> Vec<Option<usize>> {
    let mut cycles: Vec<_> = rx_input_inputs.iter().map(|_| None).collect();
    let mut n = 1;

    while cycles.iter().any(|cycle| cycle.is_none()) {
        let mut q = VecDeque::new();
        q.push_back(("button".to_string(), "broadcaster".to_string(), false));

        while let Some((sender, recipient, pulse)) = q.pop_front() {
            if let Some(next_pulse) = transmit_pulse(&sender, &recipient, pulse, modules) {
                for destination in &modules.get(&recipient).unwrap().destinations {
                    q.push_back((recipient.clone(), destination.clone(), next_pulse));
                }
            }
            if n > 1 {
                for i in 0..rx_input_inputs.len() {
                    if cycles[i].is_none() {
                        let module = modules.get(&rx_input_inputs[i]).unwrap();
                        match &module.modtype {
                            ModuleType::Conjunction(inputs) => {
                                if !inputs.values().all(|&saved| saved) {
                                    cycles[i] = Some(n);
                                }
                            }
                            _ => panic!(),
                        }
                    }
                }
            }
        }

        n += 1;
    }

    cycles
}

fn transmit_pulse(
    sender: &String,
    recipient: &String,
    pulse: bool,
    modules: &mut HashMap<String, Module>,
) -> Option<bool> {
    if let Some(module) = modules.get_mut(recipient) {
        let next_pulse: Option<bool> = match module.modtype {
            ModuleType::FlipFlop(ref mut on) => {
                if pulse {
                    None
                } else {
                    *on = !*on;
                    Some(*on)
                }
            }
            ModuleType::Conjunction(ref mut inputs) => {
                inputs.insert(sender.clone(), pulse);
                Some(!inputs.values().all(|&saved| saved))
            }
            ModuleType::Broadcast => Some(pulse),
        };

        next_pulse
    } else {
        None
    }
}

fn push_button(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut high_pulse_count = 0;
    let mut low_pulse_count = 0;

    let mut q = VecDeque::new();
    q.push_back(("button".to_string(), "broadcaster".to_string(), false));

    while let Some((sender, recipient, pulse)) = q.pop_front() {
        if pulse {
            high_pulse_count += 1;
        } else {
            low_pulse_count += 1;
        }

        if let Some(next_pulse) = transmit_pulse(&sender, &recipient, pulse, modules) {
            for destination in &modules.get(&recipient).unwrap().destinations {
                q.push_back((recipient.clone(), destination.clone(), next_pulse));
            }
        }
    }

    (high_pulse_count, low_pulse_count)
}

fn reset_modules(modules: &mut HashMap<String, Module>) {
    for module in modules.values_mut() {
        match module.modtype {
            ModuleType::FlipFlop(ref mut on) => *on = false,
            ModuleType::Conjunction(ref mut inputs) => {
                for (_, saved) in inputs {
                    *saved = false;
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let mut modules: HashMap<String, Module> = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (name, destinations) = line.split_once(" -> ").expect("error parsing input");
            let destinations: Vec<String> = destinations
                .split(", ")
                .map(|destination| destination.to_string())
                .collect();
            let (name, modtype) = match name {
                "broadcaster" => (name, ModuleType::Broadcast),
                _ => (
                    &name[1..],
                    if name.starts_with("%") {
                        ModuleType::FlipFlop(false)
                    } else if name.starts_with("&") {
                        ModuleType::Conjunction(HashMap::new())
                    } else {
                        panic!("invalid module")
                    },
                ),
            };

            (
                name.to_string(),
                Module {
                    modtype,
                    destinations,
                },
            )
        })
        .collect();

    let conjunctions: HashSet<String> = modules
        .iter()
        .filter(|(_, module)| match &module.modtype {
            ModuleType::Conjunction(_) => true,
            _ => false,
        })
        .map(|(name, _)| name.clone())
        .collect();
    let mut conjunction_inputs: HashMap<String, Vec<String>> = HashMap::new();
    for (name, module) in &modules {
        for destination in &module.destinations {
            if conjunctions.contains(destination) {
                conjunction_inputs
                    .entry(destination.clone())
                    .or_insert(Vec::new())
                    .push(name.clone());
            }
        }
    }
    for (name, input_names) in conjunction_inputs {
        let module = modules.get_mut(&name).unwrap();
        match module.modtype {
            ModuleType::Conjunction(ref mut inputs) => {
                for input_name in input_names {
                    inputs.insert(input_name, false);
                }
            }
            _ => unreachable!(),
        }
    }

    let mut high_count = 0;
    let mut low_count = 0;
    for _ in 0..1000 {
        let (hc, lc) = push_button(&mut modules);
        high_count += hc;
        low_count += lc;
    }

    println!("part 1: {}", high_count * low_count);

    reset_modules(&mut modules);

    let rx_input_inputs: Vec<String> = modules
        .iter()
        .filter(|(_, module)| module.destinations.contains(&"rx".to_string()))
        .map(|(_, module)| match &module.modtype {
            ModuleType::Conjunction(inputs) => inputs.keys().map(|k| k.clone()),
            _ => panic!(),
        })
        .flatten()
        .collect();
    let cycles = find_cycles(&rx_input_inputs, &mut modules);
    println!(
        "part 2: {}",
        cycles
            .iter()
            .map(|x| x.unwrap())
            .reduce(|acc, x| (acc * x) / gcd(acc, x))
            .unwrap()
    );
}
