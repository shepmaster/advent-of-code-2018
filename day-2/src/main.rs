use std::collections::BTreeMap;

static INPUT: &str = include_str!("../input.txt");

fn main() {
    let id_counts = INPUT.lines().map(|id| {
        let mut counts = BTreeMap::new();
        for c in id.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        counts
    });

    let mut has_2 = 0;
    let mut has_3 = 0;

    for counts in id_counts {
        if counts.values().any(|&c| c == 2) { has_2 += 1 }
        if counts.values().any(|&c| c == 3) { has_3 += 1 }
    }

    println!("Checksum: {} * {} = {}", has_2, has_3, has_2 * has_3);
}
