use regex::Regex;

static INPUT: &str = include_str!("../input.txt");

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let config = config()?;

    println!("{}", config.run_game());

    Ok(())
}

#[derive(Debug, Copy, Clone)]
struct Config {
    players: usize,
    points: usize,
}

impl Config {
    fn run_game(&self) -> usize {
        let mut scores = vec![0; self.players];
        let mut board = vec![0];
        let mut current_player = 0;
        let mut current_idx: usize = 0;
        let mut current_marble = 1;

        while current_marble <= self.points {
            // print!("{}> {:3} | ", current_player + 1, current_idx);
            // for x in &board {
            //     print!("{:3} ", x);
            // }
            // println!();

            if current_marble % 23 == 0 {
                let remove_idx = (current_idx).checked_sub(7).unwrap_or(current_idx + board.len() - 7);
                let previous_marble = board.remove(remove_idx);

                scores[current_player] += current_marble + previous_marble;

                current_idx = remove_idx;
            } else {
                let insert_idx = (current_idx + 1) % board.len() + 1;

                board.insert(insert_idx , current_marble);

                current_idx = insert_idx;
            }

            current_marble += 1;
            current_player = (current_player + 1) % self.players;
        }

        scores.into_iter().max().expect("No players")
    }
}

fn config() -> Result<Config> {
    let config_re = Regex::new(
        r"(?P<players>\d+) players; last marble is worth (?P<points>\d+) points"
    ).unwrap();

    let captures = config_re.captures(INPUT).ok_or("Unable to parse input")?;
    let players = captures.name("players").ok_or("Missing players")?.as_str().parse()?;
    let points = captures.name("points").ok_or("Missing points")?.as_str().parse()?;

    Ok(Config { players, points })
}

#[test]
fn test_0() {
    assert_eq!(Config { players: 9, points: 25 }.run_game(), 32);
}

#[test]
fn test_1() {
    assert_eq!(Config { players: 10, points: 1618 }.run_game(), 8317);
}

#[test]
fn test_2() {
    assert_eq!(Config { players: 13, points: 7999 }.run_game(), 146373);
}

#[test]
fn test_3() {
    assert_eq!(Config { players: 17, points: 1104 }.run_game(), 2764);
}

#[test]
fn test_4() {
    assert_eq!(Config { players: 21, points: 6111 }.run_game(), 54718);
}

#[test]
fn test_5() {
    assert_eq!(Config { players: 30, points: 5807 }.run_game(), 37305);
}
