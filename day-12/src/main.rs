use std::{collections::{BTreeSet, BTreeMap}, mem};
use itertools::Itertools;

static INPUT: &str = include_str!("../input.txt");
const GENERATIONS: u64 = 50_000_000_000;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let mut game = config(INPUT)?;

    game.run(GENERATIONS)?;
    // println!("{}", game.history());
    println!("Sum of pots: {}", game.pot_sum());

    Ok(())
}

#[derive(Debug, Copy, Clone)]
struct PatternContext {
    offset: i64,
    generation: u64,
}

struct Game {
    state: State,
    rules: Ruleset,
    history: Vec<State>,
    patterns: BTreeMap<State, PatternContext>,
    generation: u64,
}

impl Game {
    #[cfg(test)]
    fn n_plants(&self) -> usize {
        self.state.n_plants()
    }

    fn pot_sum(&self) -> i64 {
        self.state.pot_sum()
    }

    fn run(&mut self, generations: u64) -> Result<()> {
        for g in 0..generations {
            let (prev_state, offset) = self.state.to_pattern().ok_or("no pattern")?;
            if let Some(prev_context) = self.patterns.get(&prev_state) {
                eprintln!("cycle detected, activating time warp");
                assert_eq!(offset - prev_context.offset, 1);
                assert_eq!(g - prev_context.generation, 1);
                self.state = self.state.shift_by((generations - g) as i64);
                return Ok(());
            }
            self.tick()?;
        }
        Ok(())
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
        let last_state = mem::replace(&mut self.state, State(next_state));

        let (pattern, offset) = last_state.to_pattern().ok_or("no pattern")?;
        self.patterns.insert(pattern, PatternContext { offset, generation: self.generation });
        self.history.push(last_state);
        self.generation += 1;
        Ok(())
    }

    fn history(&self) -> HistoryDisplay {
        HistoryDisplay(self.history.clone())
    }
}

use std::{cmp, fmt};

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        HistoryDisplay(vec![self.state.clone()]).fmt(f)
    }
}

struct HistoryDisplay(Vec<State>);

impl fmt::Display for HistoryDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut min = 0;
        let mut max = 100;

        for state in &self.0 {
            let (nmin, nmax) = state.bounds().map_err(|_| fmt::Error)?;
            min = cmp::min(min, nmin);
            max = cmp::max(max, nmax);
        }

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

        for state in &self.0 {
            for i in range.clone() {
                if state.plant_at(i) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

// TODO: refactor as pattern starting at 0 and an offset
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct State(BTreeSet<i64>);

impl State {
    #[cfg(test)]
    fn n_plants(&self) -> usize {
        self.0.len()
    }

    fn pot_sum(&self) -> i64 {
        self.0.iter().cloned().sum()
    }

    fn bounds(&self) -> Result<(i64, i64)> {
        self.0.iter()
            .cloned()
            .minmax()
            .into_option()
            .ok_or("State is empty")
            .map_err(Into::into)
    }

    fn plant_at(&self, idx: i64) -> bool {
        self.0.contains(&idx)
    }

    fn neighbors_of(&self, idx: i64) -> Vec<bool> {
        (idx-2..=idx+2).map(|i| self.plant_at(i)).collect()
    }

    fn to_pattern(&self) -> Option<(Self, i64)> {
        self.0.iter().next().map(|&f| {
            let state = self.0.iter().map(|v| v - f).collect();
            (State(state), f)
        })
    }

    fn shift_by(&self, delta: i64) -> Self {
        let state = self.0.iter().map(|v| v+delta).collect();
        State(state)
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
        .filter_map(|(i, c)| if c == '#' { Some(i as i64) } else { None })
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

    Ok(Game {
        state: State(state),
        rules: Ruleset(rules),
        history: Default::default(),
        patterns: Default::default(),
        generation: 0,
    })
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
