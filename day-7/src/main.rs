use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};

static INPUT: &str = include_str!("../input.txt");
const STEP_DURATION_BASE: u32 = 60;
const N_WORKERS: usize = 5;

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

    #[derive(Debug, Copy, Clone)]
    struct WorkerState {
        name: &'static str,
        time_left: u32,
    }
    let mut workers: Vec<Option<WorkerState>> = vec![None; N_WORKERS];
    let mut time = 0;

    while !graph.is_empty() || workers.iter().any(|slot| slot.is_some()) {
        for worker_slot in &mut workers {
            if let Some(worker) = worker_slot {
                worker.time_left -= 1;

                if worker.time_left == 0 {
                    for (_, bef) in &mut graph {
                        bef.remove(worker.name);
                    }

                    order.push(worker.name);

                    *worker_slot = None;
                }
            }
        }

        let available_slots = workers.iter_mut().filter(|slot| slot.is_none());

        let available: BTreeSet<_> = graph.iter().filter_map(|(&aft, bef)| {
            if bef.is_empty() { Some(aft) } else { None }
        }).collect();

        for (slot, name) in available_slots.zip(available) {
            *slot = Some(WorkerState{ name, time_left: duration(name) } );
            graph.remove(name);
        }

       time += 1;
    }

    println!("The order is '{}'", order.iter().cloned().collect::<String>());
    println!("It took {} ticks ({} seconds)", time, time - 1);

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

fn duration(s: &str) -> u32 {
    (s.chars().next().unwrap() as u8 - b'A') as u32 + 1 + STEP_DURATION_BASE
}
