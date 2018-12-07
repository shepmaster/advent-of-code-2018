use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

static INPUT: &str = include_str!("../input.txt");

type Error = Box<std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Nearest {
    One(Coord),
    Multiple,
}

impl Nearest {
    fn into_option(self) -> Option<Coord> {
        match self {
            Nearest::One(c) => Some(c),
            Nearest::Multiple => None,
        }
    }
}

fn main() -> Result<()> {
    let coords = coords()?;

    let bounds = Bounds::new(&coords)?;
    println!("Bounds of area: {:?}", bounds);

    let find_nearest = |coord| {
        // Find distance to all coordinates from current point
        let mut distances: Vec<_> = coords.iter().map(|&c| (c, distance(coord, c))).collect();

        // Find closest coordinate(s)
        distances.sort_by_key(|&(_, d)| d);
        let (nearest, distance) = distances[0];
        let n_nearest = distances.iter().filter(|&&(_, d)| d == distance).count();

        if n_nearest == 1 {
            Nearest::One(nearest)
        } else {
            Nearest::Multiple
        }
    };

    let mut grid = BTreeMap::new();
    for coord in bounds.grid_coords() {
        grid.insert(coord, find_nearest(coord));
    }

    // Any coordinate that leaks into the fringe is infinite
    let infinite_coords: BTreeSet<_> = bounds
        .fringe_coords()
        .filter_map(|coord| find_nearest(coord).into_option())
        .collect();
    let non_infinite_coords = &coords - &infinite_coords;

    let non_infinite_grid_coords = grid
        .values()
        .filter_map(|c| c.into_option())
        .filter(|c| non_infinite_coords.contains(c));

    let mut counts = BTreeMap::new();
    for coord in non_infinite_grid_coords {
        *counts.entry(coord).or_insert(0) += 1;
    }

    if let Some((coord, count)) = counts.into_iter().max_by_key(|&(_, count)| count) {
        println!("Coordinate {:?} has an area of {}", coord, count);
    }

    let within_10000 = bounds.wide_coords().filter(|&coord| {
        coords.iter().map(|&c| distance(c, coord)).sum::<i32>() < 10_000
    }).count();

    println!("There are {} coordinates with a total of 10,000 distance", within_10000);

    Ok(())
}

type Coord = (i32, i32);

fn distance(a: Coord, b: Coord) -> i32 {
    let [x0, x1] = { let mut t = [a.0, b.0]; t.sort(); t };
    let [y0, y1] = { let mut t = [a.1, b.1]; t.sort(); t };
    x1 - x0 + y1 - y0
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Bounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Bounds {
    fn new<'a>(coords: impl IntoIterator<Item = &'a Coord> + Copy) -> Result<Bounds> {
        let (&min_x, &max_x) = coords
            .into_iter()
            .map(|(x, _)| x)
            .minmax()
            .into_option()
            .ok_or("Must have one coordinate")?;
        let (&min_y, &max_y) = coords
            .into_iter()
            .map(|(_, y)| y)
            .minmax()
            .into_option()
            .ok_or("Must have one coordinate")?;
        Ok(Bounds { min_x, max_x, min_y, max_y })
    }

    fn grid_coords(&self) -> impl Iterator<Item = Coord> {
        let Bounds { min_x, max_x, min_y, max_y } = *self;
        (min_x..max_x).cartesian_product(min_y..max_y)
    }

    fn fringe_coords(&self) -> impl Iterator<Item = Coord> {
        let Bounds { mut min_x, mut max_x, mut min_y, mut max_y } = *self;
        min_x -= 1;
        min_y -= 1;
        max_x += 1;
        max_y += 1;

        let top = (min_x..max_x).map(move |x| (x, min_y));
        let bot = (min_x..max_x).map(move |x| (x, max_y));
        let lft = (min_y..max_y).map(move |y| (min_x, y));
        let rgt = (min_y..max_y).map(move |y| (max_x, y));

        top.chain(bot).chain(lft).chain(rgt)
    }

    fn wide_coords(&self) -> impl Iterator<Item = Coord> {
        let Bounds { mut min_x, mut max_x, mut min_y, mut max_y } = *self;
        min_x -= 10_000;
        min_y -= 10_000;
        max_x += 10_000;
        max_y += 10_000;

        (min_x..max_x).cartesian_product(min_y..max_y)
    }
}

fn coords() -> Result<BTreeSet<Coord>> {
    INPUT
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut parts = l.splitn(2, ',').fuse();
            let x = parts.next().ok_or("Missing X")?.trim().parse()?;
            let y = parts.next().ok_or("Missing Y")?.trim().parse()?;
            Ok((x, y))
        })
        .collect()
}
