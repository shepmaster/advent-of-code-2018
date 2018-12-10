use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};

static INPUT: &str = include_str!("../input.txt");

type Error = Box<std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let mut graph = BTreeMap::new();

    let dependencies = dependencies()?;

    for (bef, aft) in dependencies {
        graph.entry(aft).or_insert_with(BTreeSet::new).insert(bef);
    }

    // Add dummy nodes for any missing steps
    {
        let mut steps = BTreeSet::new();

        for (&aft, bef) in &graph {
            steps.insert(aft);
            steps.extend(bef);
        }

        for step in steps {
            graph.entry(step).or_insert_with(BTreeSet::new);
        }
    }

    let mut order: Vec<_> = Vec::new();

    while !graph.is_empty() {
        let available: BTreeSet<_> = graph.iter().filter_map(|(&aft, bef)| {
            if bef.is_empty() { Some(aft) } else { None }
        }).collect();

        let next = available.into_iter().next().ok_or("Unable to make progress")?;

        graph.remove(next);
        for (_, bef) in graph.iter_mut() {
            bef.remove(next);
        }

        order.push(next);
    }

    println!("The order is '{}'", order.iter().cloned().collect::<String>());

    Ok(())
}

fn dependencies() -> Result<Vec<(&'static str, &'static str)>> {
    let dep_regex = Regex::new(r"Step (\w+) must be finished before step (\w+) can begin.").unwrap();

    INPUT.lines().map(|line| {
        let captures = dep_regex.captures(line).ok_or("Could not apply regex to line")?;
        let bef = captures.get(1).ok_or("Did not find before step")?;
        let aft = captures.get(2).ok_or("Did not find after step")?;

        Ok((bef.as_str(), aft.as_str()))
    }).collect()
}
