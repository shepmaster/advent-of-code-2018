use itertools::Itertools;
use std::{iter, collections::BTreeMap, mem};

const SERIAL_NUMBER: i32 = 5153;

fn main() {
    let max_top_left = max_max_top_left(SERIAL_NUMBER);
    println!("Top-left: {:?}", max_top_left);
}

type Coord = (i32, i32);
type Grid = BTreeMap<Coord, i32>;

fn grid(serial_number: i32) -> Grid {
    let coords = (1..=300).cartesian_product(1..=300);
    coords.map(|coord| {
        let power = power_level(coord.0, coord.1, serial_number);
        (coord, power)
    }).collect()
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct MetaSquare {
    coord: Coord,
    power: i32,
    width: i32,
}

impl MetaSquare {
    fn frontier(&self) -> impl Iterator<Item = Coord> {
        let Self { coord: (x, y), width, .. } = *self;

        let rgt = (x..x + width).map(move |x| (x, y + width));
        let bot = (y..y + width).map(move |y| (x + width, y));
        let cor = iter::once((x + width, y + width));

        rgt.chain(bot).chain(cor)
    }

    fn grow(&self, diff: i32) -> Self {
        let Self { coord, width, power } = *self;
        Self { coord, width: width + 1, power: power + diff }
    }
}

fn max_max_top_left(serial_number: i32) -> Option<MetaSquare> {
    let grid = grid(serial_number);

    let mut all_steps = Vec::new();

    let mut current_step: Vec<_> = grid
        .iter()
        .map(|(&coord, &power)| MetaSquare { coord, power, width: 1 })
        .collect();

    for width in 0..300 {
        eprintln!("{}", width);

        let next_step = current_step.iter().flat_map(|meta_square| {
            meta_square
                .frontier()
                .map(|coord| grid.get(&coord).cloned())
                .try_fold(0, |acc, v| v.map(|v| v + acc))
                .map(|diff| meta_square.grow(diff))
        }).collect();

        all_steps.extend(mem::replace(&mut current_step, next_step));
    }

    all_steps.into_iter().max_by_key(|ms| ms.power)
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Square {
    coord: Coord,
    power: i32,
}

#[cfg(test)]
fn max_top_left(serial_number: i32, width: i32) -> Option<Square> {
    let grid = grid(serial_number);
    max_top_left_inner(&grid, width)
}

#[cfg(test)]
fn max_top_left_inner(grid: &Grid, width: i32) -> Option<Square> {
    let coords = (1..300 - width).cartesian_product(1..300 - width);
    coords.map(|(x, y)| {
        let three_by_three = (x..x + width).cartesian_product(y..y + width);
        let power = three_by_three
            .flat_map(|coord| grid.get(&coord))
            .sum::<i32>();
        Square { coord: (x, y), power }
    })
        .max_by_key(|square| square.power)
}

#[test]
fn max_top_left_0() {
    assert_eq!(max_top_left(18, 3), Some(Square { coord: (33, 45), power: 29 }));
}

#[test]
fn max_top_left_1() {
    assert_eq!(max_top_left(42, 3), Some(Square { coord: (21, 61), power: 30 }));
}

fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;
    let hundreds = power_level / 100 % 10;
    hundreds - 5
}

#[test]
fn power_level_0() {
    assert_eq!(power_level(3, 5, 8), 4);
}

#[test]
fn power_level_1() {
    assert_eq!(power_level(122,  79, 57), -5);
}

#[test]
fn power_level_2() {
    assert_eq!(power_level(217, 196, 39), 0);
}

#[test]
fn power_level_3() {
    assert_eq!(power_level(101, 153, 71), 4);
}
