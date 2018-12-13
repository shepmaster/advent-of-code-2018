use std::collections::BTreeSet;
use regex::Regex;
use itertools::Itertools;

static INPUT: &str = include_str!("../input.txt");

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let points = points()?;

    let mut grid = Grid(points);
    let mut min_grid = grid.clone();

    let max_area = grid.area()?;
    let mut min_area = max_area;
    let mut step = 0;
    let mut min_step = step;


    loop {
        let current_area = grid.area()?;

        if current_area < min_area {
            min_area = current_area;
            min_step = step;
            min_grid = grid.clone();
        }

        if current_area > max_area {
            break;
        }

        grid.step();
        step += 1;
    }
    println!("Total of {} steps", step);
    println!("Max area is {}", max_area);
    println!("Min area is {} at step {}", min_area, min_step);
    println!("{}", min_grid);

    Ok(())
}

#[derive(Debug, Clone)]
struct Grid(Vec<Point>);

impl Grid {
    fn area(&self) -> Result<i64> {
        let ((x0, x1), (y0, y1)) = self.bounds()?;
        Ok((x1-x0) * (y1-y0))
    }

    fn bounds(&self) -> Result<((i64, i64), (i64, i64))> {
        let x = self.0.iter().map(|pt| pt.x).minmax().into_option().ok_or("No points")?;
        let y = self.0.iter().map(|pt| pt.y).minmax().into_option().ok_or("No points")?;

        Ok((x, y))
    }

    fn step(&mut self) {
        for pt in &mut self.0 {
            pt.step();
        }
    }
}

use std::fmt;

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ((x0, x1), (y0, y1)) = self.bounds().map_err(|_| fmt::Error)?;
        let set: BTreeSet<_> = self.0.iter().map(|pt| (pt.x, pt.y)).collect();

        for y in y0..=y1 {
            for x in x0..=x1 {
                if set.contains(&(x, y)) {
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

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

impl Point {
    fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}

fn points() -> Result<Vec<Point>> {
    // position=<-43587, -21695> velocity=< 4,  2>
    let point_re = Regex::new(
        r"position=<\s*(?P<x>-?\d+),\s*(?P<y>-?\d+)> velocity=<\s*(?P<dx>-?\d+),\s*(?P<dy>-?\d+)>"
    ).unwrap();

    INPUT.lines().map(|l| {
        let captures = point_re.captures(l).ok_or("Unable to parse input")?;
        let x = captures.name("x").ok_or("Missing X")?.as_str().parse()?;
        let y = captures.name("y").ok_or("Missing Y")?.as_str().parse()?;
        let dx = captures.name("dx").ok_or("Missing dX")?.as_str().parse()?;
        let dy = captures.name("dy").ok_or("Missing dY")?.as_str().parse()?;

        Ok(Point { x, y, dx, dy })
    }).collect()
}
