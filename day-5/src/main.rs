use std::collections::{BTreeMap, BTreeSet};

static INPUT: &str = include_str!("../input.txt");

fn main() {
    let polymer: Vec<_> = INPUT.trim().chars().collect();

    let units: BTreeSet<_> = polymer.iter().map(char::to_ascii_uppercase).collect();

    let all_reactions: BTreeMap<_, _> = units.iter().map(|&c| (c, complete_react(remove_unit(&polymer, c)))).collect();

    let min_length = all_reactions.values().map(|r| r.len()).min();

    println!("Resulting polymer is {:?} units", min_length);
}

fn remove_unit(polymer: &[char], unit: char) -> Vec<char> {
    let mut polymer = polymer.to_owned();
    polymer.retain(|c| !c.eq_ignore_ascii_case(&unit));
    polymer
}

fn complete_react(mut polymer: Vec<char>) -> Vec<char> {
    loop {
        let start_len = polymer.len();
        polymer = react(polymer);
        if polymer.len() == start_len {
            return polymer;
        }
    }
}

fn react(polymer: Vec<char>) -> Vec<char> {
    let mut i = 0;
    let mut next = Vec::new();

    loop {
        match (polymer.get(i), polymer.get(i+1)) {
            (Some(&a), Some(&b)) if opposing_polarity(a, b) => i += 2,
            (Some(&a), _) => {
                next.push(a);
                i += 1;
            }
            _ => break,
        }
    }

    next
}

fn opposing_polarity(a: char, b: char) -> bool {
    a.eq_ignore_ascii_case(&b) && a != b
}
