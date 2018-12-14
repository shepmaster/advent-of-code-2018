use std::collections::{BTreeSet, BTreeMap};
use itertools::Itertools;

static INPUT: &str = include_str!("../input.txt");

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let mut game = config(INPUT)?;

    for _ in 0..20 {
        game.tick()?;
    }

    println!("Sum of pots: {}", game.pot_sum());

    Ok(())
}

struct Game {
    state: State,
    rules: Ruleset,
}

impl Game {
    #[cfg(test)]
    fn n_plants(&self) -> usize {
        self.state.n_plants()
    }

    fn pot_sum(&self) -> i32 {
        self.state.pot_sum()
    }

    fn tick(&mut self) -> Result<()> {
        let (x_min, x_max) = self.state.bounds()?;
        let next_state = (x_min-2..=x_max+2).filter_map(|i| {
            let neighbors = self.state.neighbors_of(i);
            if self.rules.for_neighbors(&neighbors) {
                Some(i)
            } else {
                None
            }
        }).collect();
        self.state = State(next_state);
        Ok(())
    }
}

use std::fmt;

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (min, max) = self.state.bounds().map_err(|_| fmt::Error)?;
        let range = min..=max;


        for i in range.clone() {
            if i % 10 == 0 {
                write!(f, "{}", i / 10)?;
            } else {
                write!(f, " ")?;
            }
        }
        writeln!(f)?;

        for i in range.clone() {
            if i % 10 == 0 {
                write!(f, "0")?;
            } else {
                write!(f, " ")?;
            }
        }
        writeln!(f)?;

        for i in range {
            if self.state.plant_at(i) {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        writeln!(f)?;

        Ok(())
    }
}

#[derive(Debug)]
struct State(BTreeSet<i32>);

impl State {
    #[cfg(test)]
    fn n_plants(&self) -> usize {
        self.0.len()
    }

    fn pot_sum(&self) -> i32 {
        self.0.iter().cloned().sum()
    }

    fn bounds(&self) -> Result<(i32, i32)> {
        self.0.iter()
            .cloned()
            .minmax()
            .into_option()
            .ok_or("State is empty")
            .map_err(Into::into)
    }

    fn plant_at(&self, idx: i32) -> bool {
        self.0.contains(&idx)
    }

    fn neighbors_of(&self, idx: i32) -> Vec<bool> {
        (idx-2..=idx+2).map(|i| self.plant_at(i)).collect()
    }
}

#[derive(Debug)]
struct Ruleset(BTreeMap<Vec<bool>, bool>);

impl Ruleset {
    fn for_neighbors(&self, neighbors: &[bool]) -> bool {
        self.0.get(neighbors).cloned().unwrap_or(false)
    }
}

fn config(input: &str) -> Result<Game> {
    let mut l = input.lines().fuse();
    let state = l.next().ok_or("No initial state")?;
    let state = state.split(":").nth(1).ok_or("No initial state")?;
    let state = state
        .trim()
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if c == '#' { Some(i as i32) } else { None })
        .collect();

    l.next();
    let rules = l.map(|l| {
        let mut l = l.split("=>").fuse();
        let neighbors = l.next().ok_or("No rule neighbors")?;
        let neighbors = neighbors.trim().chars().map(|c| c == '#').collect();
        let next = l.next().ok_or("No rule next")?;
        let next = next.trim().chars().next() == Some('#');

        Ok((neighbors, next))
    }).collect::<Result<_>>()?;

    Ok(Game { state: State(state), rules: Ruleset(rules) })
}

#[test]
fn example_0() -> Result<()> {
    let mut game = config(r"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #")?;

    assert_eq!(game.n_plants(), 11, );

    let expected = [
        7, 11, 9, 11, 9,
        12, 11, 14, 12, 14,
        10, 14, 11, 14, 11,
        14, 12, 18, 20, 19,
    ];

    for &e in &expected {
        game.tick()?;
        assert_eq!(game.n_plants(), e);
    }

    assert_eq!(game.pot_sum(), 325);

    Ok(())
}
