use regex::Regex;
use itertools::Itertools;
use std::collections::BTreeMap;

static INPUT: &str = include_str!("../input.txt");

type Error = Box<std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl Claim {
    fn squares<'a>(&'a self) -> impl Iterator<Item = (u32, u32)> + 'a {
        let Claim { x, y, .. } = *self;
        let x = (0..self.w).map(move |i| i + x);
        let y = (0..self.h).map(move |i| i + y);

        x.cartesian_product(y)
    }
}

fn main() -> Result<()> {
    let claims = claims()?;

    let mut cloth = BTreeMap::new();

    for claim in &claims {
        for coord in claim.squares() {
            *cloth.entry(coord).or_insert(0) += 1;
        }
    }

    let contested = cloth.iter().filter(|(_, &count)| count >= 2).count();
    println!("There are {} contested squares", contested);

    Ok(())
}

fn claims() -> Result<Vec<Claim>> {
    // #123 @ 3,2: 5x4
    let claim_regex = Regex::new(r"(?x)
        \#
        (?P<id>\d+)
        \s+
        @
        \s+
        (?P<x>\d+),(?P<y>\d+)
        :
        \s+
        (?P<w>\d+)x(?P<h>\d+)
    ").unwrap();

    INPUT.lines().map(|l| {
        let captures = claim_regex.captures(l).ok_or_else(|| "No matching captures")?;

        let id = captures.name("id").ok_or_else(|| "No ID")?;
        let id = id.as_str().parse()?;

        let x = captures.name("x").ok_or_else(|| "No X")?;
        let x = x.as_str().parse()?;

        let y = captures.name("y").ok_or_else(|| "No Y")?;
        let y = y.as_str().parse()?;

        let w = captures.name("w").ok_or_else(|| "No Width")?;
        let w = w.as_str().parse()?;

        let h = captures.name("h").ok_or_else(|| "No Height")?;
        let h = h.as_str().parse()?;

        Ok(Claim { id, x, y, w, h })
    }).collect()
}
