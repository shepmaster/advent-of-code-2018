use std::collections::BTreeMap;

static INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut game = initial(INPUT);
    let crash_point = game.run();
    println!("{:?}", crash_point);
}

#[derive(Debug, Copy, Clone)]
enum Track {
    Horizontal,
    Vertical,
    Northwest,
    Northeast,
    Intersection,
}

#[derive(Debug, Copy, Clone)]
struct Cart {
    direction: Direction,
    turn: Turn,
}

impl Cart {
    fn new(direction: Direction) -> Self {
        Self { direction, turn: Turn::Left }
    }

    fn next_coord(&self, coord: Coord) -> Coord {
        self.direction.next_coord(coord)
    }

    fn take_turn(&self, next_track: Track) -> Self {
        use self::Track::*;
        use self::Direction::*;

        let (direction, turn) = match (self.direction, next_track) {
            (South, Vertical) |
            (North, Vertical) |
            (West, Horizontal) |
            (East, Horizontal) => (self.direction, self.turn),

            (South, Horizontal) |
            (North, Horizontal) |
            (West, Vertical) |
            (East, Vertical) => panic!("Track turned 90 degrees"),

            (North, Northwest) => (West, self.turn),
            (North, Northeast) => (East, self.turn),
            (South, Northwest) => (East, self.turn),
            (South, Northeast) => (West, self.turn),

            (West, Northwest) => (North, self.turn),
            (West, Northeast) => (South, self.turn),
            (East, Northwest) => (South, self.turn),
            (East, Northeast) => (North, self.turn),

            (South, Intersection) |
            (North, Intersection) |
            (East, Intersection) |
            (West, Intersection) => self.turn_at_intersection(),

            (Crash, _) => unimplemented!("Crash, _"),
        };

        Cart { direction, turn }
    }

    fn turn_at_intersection(&self) -> (Direction, Turn) {
        use self::Direction::*;
        use self::Turn::*;
        match (self.direction, self.turn) {
            (_, Straight) => (self.direction, Right),

            (North, Left) => (West, Straight),
            (North, Right) => (East, Left),

            (East, Left) => (North, Straight),
            (East, Right) => (South, Left),

            (South, Left) => (East, Straight),
            (South, Right) => (West, Left),

            (West, Left) => (South, Straight),
            (West, Right) => (North, Left),

            (Crash, _) => (Crash, self.turn),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
    Crash,
}

impl Direction {
    fn next_coord(&self, coord: Coord) -> Coord {
        use self::Direction::*;

        let (x, y) = coord;
        match self {
            North => (x, y-1),
            South => (x, y+1),
            East =>  (x+1, y),
            West =>  (x-1, y),
            Crash => coord,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    Left,
    Straight,
    Right,
}

type Coord = (usize, usize);

#[derive(Debug)]
struct Game {
    tracks: Tracks,
    carts: Carts,
}

impl Game {
    fn run(&mut self) -> Coord {
        loop {
            if let Err(coord) = self.step() {
                return coord;
            }
        }
    }

    fn step(&mut self) -> Result<(), Coord> {
        let mut old_coords: Vec<_> = self.carts.0.keys().cloned().collect();
        old_coords.reverse();

        let mut next_carts = Carts(BTreeMap::new());

        while let Some(coord) = old_coords.pop() {
            let cart = self.carts.0.remove(&coord).expect("Cart wasn't at coordinate");
            let next_coord = cart.next_coord(coord);

            if self.carts.at(next_coord).is_some() {
                return Err(next_coord);
            }
            if next_carts.at(next_coord).is_some() {
                return Err(next_coord);
            }

            let next_track = self.tracks.at(next_coord)
                .unwrap_or_else(|| panic!("Cart has gone off the tracks at {:?}", next_coord));

            let next_cart = cart.take_turn(next_track);

            next_carts.0.insert(next_coord, next_cart);
        }

        self.carts = next_carts;
        Ok(())
    }
}

use std::fmt;
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::cmp::{min, max};

        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for &(x, y) in self.tracks.0.keys() {
            min_x = min(x, min_x);
            max_x = max(x, max_x);
            min_y = min(y, min_y);
            max_y = max(y, max_y);
        }

        use self::Direction::*;
        use self::Track::*;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let coord = (x, y);
                let c = match self.carts.at(coord) {
                    Some(c) => match c.direction {
                        North => '^',
                        East => '>',
                        South => 'v',
                        West => '<',
                        Crash => 'X',
                    }
                    None => match self.tracks.at(coord) {
                        Some(t) => match t {
                            Vertical => '|',
                            Horizontal => '-',
                            Northwest => '\\',
                            Northeast => '/',
                            Intersection => '+',
                        },
                        None => ' ',
                    }
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Tracks(BTreeMap<Coord, Track>);

impl Tracks {
    fn at(&self, coord: Coord) -> Option<Track>{
        self.0.get(&coord).cloned()
    }
}

#[derive(Debug)]
struct Carts(BTreeMap<Coord, Cart>);

impl Carts {
    fn at(&self, coord: Coord) -> Option<Cart> {
        self.0.get(&coord).cloned()
    }
}

fn initial(input: &str) -> Game {
    let mut track = BTreeMap::new();
    let mut carts = BTreeMap::new();

    for (y, line) in input.lines().filter(|l| !l.trim().is_empty()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = (x, y);

            match c {
                '-'  => { track.insert(coord, Track::Horizontal); }
                '|'  => { track.insert(coord, Track::Vertical); }
                '\\' => { track.insert(coord, Track::Northwest); }
                '/'  => { track.insert(coord, Track::Northeast); }
                '+'  => { track.insert(coord, Track::Intersection); }
                '^'  => {
                    track.insert(coord, Track::Vertical);
                    carts.insert(coord, Cart::new(Direction::North));
                }
                'v'  => {
                    track.insert(coord, Track::Vertical);
                    carts.insert(coord, Cart::new(Direction::South));
                }
                '>'  => {
                    track.insert(coord, Track::Horizontal);
                    carts.insert(coord, Cart::new(Direction::East));
                }
                '<'  => {
                    track.insert(coord, Track::Horizontal);
                    carts.insert(coord, Cart::new(Direction::West));
                }
                _    => { /* no-op */ }
            }
        }
    }

    Game { tracks: Tracks(track), carts: Carts(carts) }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST: &str =
r"
/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/
";

    #[test]
    fn example_0() {
        let mut game = initial(TEST);
        assert_eq!(game.run(), (7, 3));
    }
}
