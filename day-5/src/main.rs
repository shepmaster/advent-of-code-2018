static INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut polymer: Vec<_> = INPUT.trim().chars().collect();

    loop {
        let start_len = polymer.len();
        polymer = react(polymer);
        if polymer.len() == start_len { break }
    }

    println!("Resulting polymer is {} units", polymer.len());
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
