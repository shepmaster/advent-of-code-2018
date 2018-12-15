const INPUT: usize = 652601;

fn main() {
    let mut rb = RecipeBoard::new();

    for d in rb.ten_after(INPUT) {
        print!("{}", d);
    }
    println!();
}

struct RecipeBoard {
    scores: Vec<u8>,
    elf1: usize,
    elf2: usize,
}

impl RecipeBoard {
    fn new() -> Self {
        Self {
            scores: vec![3, 7],
            elf1: 0,
            elf2: 1,
        }
    }

    fn ten_after(&mut self, n_recipes: usize) -> &[u8] {
        self.make_total_recipes(n_recipes + 10);
        &self.scores[n_recipes..][..10]
    }

    fn make_total_recipes(&mut self, n_recipes: usize) {
        while self.scores.len() < n_recipes {
            self.step();
        }
        self.scores.truncate(n_recipes);
        assert_eq!(self.scores.len(), n_recipes, "Didn't make the right number of recipes");
    }

    fn step(&mut self) {
        let e1 = self.scores[self.elf1];
        let e2 = self.scores[self.elf2];
        let sum = e1 + e2;

        self.scores.extend(digits(sum));

        self.elf1 += e1 as usize + 1;
        self.elf1 %= self.scores.len();

        self.elf2 += e2 as usize + 1;
        self.elf2 %= self.scores.len();
    }
}

fn digits(number: u8) -> impl Iterator<Item = u8> {
    let d: Vec<u8> = number.to_string().chars().flat_map(|c| c.to_digit(10)).map(|d| d as u8).collect();
    d.into_iter()
}

#[test]
fn example_0() {
    let mut rb = RecipeBoard::new();
    assert_eq!(rb.scores, [3, 7]);

    let expected: &[&[u8]] = &[
        &[3, 7, 1, 0],
        &[3, 7, 1, 0, 1, 0],
        &[3, 7, 1, 0, 1, 0, 1],
        &[3, 7, 1, 0, 1, 0, 1, 2, ],
        &[3, 7, 1, 0, 1, 0, 1, 2, 4],
        &[3, 7, 1, 0, 1, 0, 1, 2, 4, 5],
        &[3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1],
        &[3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5],
        &[3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8],
        &[3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9],
        &[3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6],
        &[3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7],
        &[3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7],
        &[3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9],
        &[3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9, 2],
    ];

    for &expected in expected {
        rb.step();
        assert_eq!(rb.scores, expected);
    }
}

#[test]
fn example_1() {
    let mut rb = RecipeBoard::new();
    assert_eq!(rb.ten_after(9), [5, 1, 5, 8, 9, 1, 6, 7, 7, 9]);
}

#[test]
fn example_2() {
    let mut rb = RecipeBoard::new();
    assert_eq!(rb.ten_after(5), [0, 1, 2, 4, 5, 1, 5, 8, 9, 1]);
}

#[test]
fn example_3() {
    let mut rb = RecipeBoard::new();
    assert_eq!(rb.ten_after(18), [9, 2, 5, 1, 0, 7, 1, 0, 8, 5]);
}

#[test]
fn example_4() {
    let mut rb = RecipeBoard::new();
    assert_eq!(rb.ten_after(2018), [5, 9, 4, 1, 4, 2, 9, 8, 8, 2]);
}
