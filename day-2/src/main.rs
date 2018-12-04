use itertools::Itertools;
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

    let ids = INPUT.lines();
    // Eww, O(N^2)
    let mut z = ids.clone().cartesian_product(ids).filter_map(|(a, b)| {
        let mut same = Vec::new();
        let mut diff = 0;

        for (a, b) in a.chars().zip(b.chars()) {
            if a == b {
                same.push(a);
            } else {
                diff += 1;
            }

            if diff > 1 { return None }
        }

        match diff {
            0 => None,
            1 => Some(same.into_iter().collect::<String>()),
            _ => None,
        }
    });

    if let Some(shared) = z.next() {
        println!("Shared: {}", shared);
    }
}
