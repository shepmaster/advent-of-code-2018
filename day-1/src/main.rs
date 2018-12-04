static INPUT: &str = include_str!("../input.txt");

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let sum: Result<i128, _> = INPUT.lines().map(str::parse::<i128>).sum();
    println!("sum: {:?}", sum?);
    Ok(())
}
