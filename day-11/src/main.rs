use itertools::Itertools;
use std::collections::BTreeMap;

const SERIAL_NUMBER: i32 = 5153;

fn main() {
    let max_top_left = max_top_left(SERIAL_NUMBER);
    println!("Top-left: {:?}", max_top_left);
}

fn max_top_left(serial_number: i32) -> Option<(i32, i32)> {
    let coords = (1..=300).cartesian_product(1..=300);
    let grid: BTreeMap<_, _> = coords.map(|coord| {
        let power = power_level(coord.0, coord.1, serial_number);
        (coord, power)
    }).collect();

    let coords = (1..=298).cartesian_product(1..=298);
    coords.max_by_key(|&(x, y)| {
        let three_by_three = (x..=x+2).cartesian_product(y..=y+2);
        three_by_three
            .flat_map(|coord| grid.get(&coord))
            .sum::<i32>()
    })
}

#[test]
fn max_top_left_0() {
    assert_eq!(max_top_left(18), Some((33, 45)));
}

#[test]
fn max_top_left_1() {
    assert_eq!(max_top_left(42), Some((21, 61)));
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
