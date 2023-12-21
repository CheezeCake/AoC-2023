use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
enum Comparison {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
struct Condition {
    rating: String,
    cmp: Comparison,
    value: u32,
}

impl Condition {
    fn apply(&self, part: &Part) -> bool {
        let value = *part.ratings.get(&self.rating).unwrap();
        match self.cmp {
            Comparison::LessThan => value < self.value,
            Comparison::GreaterThan => value > self.value,
        }
    }
}

struct ParseConditionError;

impl FromStr for Condition {
    type Err = ParseConditionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rating = s[0..1].to_string();
        let cmp = match &s[1..2] {
            "<" => Comparison::LessThan,
            ">" => Comparison::GreaterThan,
            _ => return Err(ParseConditionError),
        };
        let value: u32 = s[2..].parse().map_err(|_| ParseConditionError)?;

        Ok(Condition { rating, cmp, value })
    }
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    destination: String,
}

impl Rule {
    fn apply(&self, part: &Part) -> bool {
        if let Some(cond) = &self.condition {
            cond.apply(part)
        } else {
            true
        }
    }
}

struct ParseRuleError;

impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((condition, destination)) = s.split_once(':') {
            Ok(Rule {
                condition: Some(condition.parse().map_err(|_| ParseRuleError)?),
                destination: destination.to_string(),
            })
        } else {
            Ok(Rule {
                condition: None,
                destination: s.to_string(),
            })
        }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn apply(&self, part: &Part) -> String {
        for rule in &self.rules {
            if rule.apply(part) {
                return rule.destination.clone();
            }
        }
        unreachable!()
    }
}

#[derive(Debug)]
struct ParseWorkflowError;

impl FromStr for Workflow {
    type Err = ParseWorkflowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules: Vec<Rule> = Vec::new();

        for rule in s
            .strip_prefix('{')
            .and_then(|s| s.strip_suffix('}'))
            .ok_or(ParseWorkflowError)?
            .split(',')
        {
            rules.push(rule.parse::<Rule>().map_err(|_| ParseWorkflowError)?);
        }

        Ok(Workflow { rules })
    }
}

#[derive(Debug)]
struct Part {
    ratings: HashMap<String, u32>,
}

#[derive(Debug)]
struct ParsePartError;

impl FromStr for Part {
    type Err = ParsePartError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ratings = HashMap::new();

        for rating in s
            .strip_prefix('{')
            .and_then(|s| s.strip_suffix('}'))
            .ok_or(ParsePartError)?
            .split(',')
        {
            let (rating, value) = rating.split_once('=').ok_or(ParsePartError)?;
            let value = value.parse::<u32>().map_err(|_| ParsePartError)?;
            ratings.insert(rating.to_string(), value);
        }

        Ok(Part { ratings })
    }
}

fn accepted(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut workflow = "in".to_string();

    while !workflow.eq("A") && !workflow.eq("R") {
        workflow = workflows.get(&workflow).unwrap().apply(part);
    }

    workflow.eq("A")
}

fn solve(
    invalid_rating_values: &mut HashMap<String, Vec<(u32, u32)>>,
    workflow: &String,
    workflows: &HashMap<String, Workflow>,
) -> usize {
    if workflow.eq("A") {
        let mut combinations = 1;
        for rating in ["x", "m", "a", "s"] {
            let mut invalid = HashSet::new();
            for invalid_range in invalid_rating_values.get(&rating.to_string()).unwrap() {
                for value in invalid_range.0..=invalid_range.1 {
                    invalid.insert(value);
                }
            }

            let valid = 4000 - invalid.len();
            combinations *= valid;
        }

        return combinations;
    } else if workflow.eq("R") {
        return 0;
    }

    let mut result = 0;
    let workflow = workflows.get(workflow).unwrap();

    for rule in &workflow.rules {
        if let Some(condition) = &rule.condition {
            let (valid, invalid) = match condition.cmp {
                Comparison::LessThan => ((1, condition.value - 1), (condition.value, 4000)),
                Comparison::GreaterThan => ((condition.value + 1, 4000), (1, condition.value)),
            };

            invalid_rating_values
                .get_mut(&condition.rating)
                .unwrap()
                .push(invalid);
            result += solve(invalid_rating_values, &rule.destination, workflows);
            invalid_rating_values
                .get_mut(&condition.rating)
                .unwrap()
                .pop();
            invalid_rating_values
                .get_mut(&condition.rating)
                .unwrap()
                .push(valid);
        } else {
            result += solve(invalid_rating_values, &rule.destination, workflows);
        }
    }

    for rule in &workflow.rules {
        if let Some(condition) = &rule.condition {
            invalid_rating_values
                .get_mut(&condition.rating)
                .unwrap()
                .pop();
        }
    }

    result
}

fn main() {
    let mut workflows = HashMap::new();

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => panic!("unexpected end of input"),
            Ok(1) => break,
            Ok(_) => {
                let brace = input.find('{').expect("error parsing input");
                let name = input[..brace].to_string();
                let workflow: Workflow = input[brace..]
                    .trim_end()
                    .parse()
                    .expect("error parsing workflow");
                workflows.insert(name, workflow);
            }
            Err(e) => panic!("error reading input: {}", e),
        }
    }

    let mut parts = Vec::new();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(e) => panic!("error reading input: {}", e),
            Ok(0) => break,
            Ok(_) => {}
        }

        let part: Part = input.trim().parse().expect("error parsing part ratings");
        parts.push(part);
    }

    let x: u32 = parts
        .iter()
        .filter(|part| accepted(part, &workflows))
        .map(|part| part.ratings.values().sum::<u32>())
        .sum();
    println!("part 1: {}", x);

    let mut invalid: HashMap<String, Vec<(u32, u32)>> = HashMap::new();
    invalid.insert("x".to_string(), Vec::new());
    invalid.insert("m".to_string(), Vec::new());
    invalid.insert("a".to_string(), Vec::new());
    invalid.insert("s".to_string(), Vec::new());
    println!(
        "part 2: {}",
        solve(&mut invalid, &"in".to_string(), &workflows,)
    );
}
