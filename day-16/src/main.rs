use itertools::Itertools;
use arrayvec::ArrayVec;

static INPUT_PART_1: &str = include_str!("../input-part1.txt");

fn main() {
    let matching = testcases(INPUT_PART_1)
        .filter(|tc| tc.candidates().count() >= 3)
        .count();
    println!("There are {} testcases with 3 or more opcode candidates", matching);
}

type Reg = usize;
type RegisterFile = [Reg; 4];

#[derive(Debug, Copy, Clone)]
enum Opcode {
    /// add register
    ///
    /// stores into register C the result of adding register A and register B.
    Addr(Reg, Reg, Reg),
    /// add immediate
    ///
    /// stores into register C the result of adding register A and value B.
    Addi(Reg, Reg, Reg),

    /// multiply register
    ///
    /// stores into register C the result of multiplying register A and register B.
    Mulr(Reg, Reg, Reg),
    /// multiply immediate
    ///
    /// stores into register C the result of multiplying register A and value B.
    Muli(Reg, Reg, Reg),

    /// bitwise AND register
    ///
    /// stores into register C the result of the bitwise AND of register A and register B.
    Banr(Reg, Reg, Reg),
    /// bitwise AND immediate
    ///
    /// stores into register C the result of the bitwise AND of register A and value B.
    Bani(Reg, Reg, Reg),

    /// bitwise OR register
    ///
    /// stores into register C the result of the bitwise OR of register A and register B.
    Borr(Reg, Reg, Reg),
    /// bitwise OR immediate
    ///
    /// stores into register C the result of the bitwise OR of register A and value B.
    Bori(Reg, Reg, Reg),

    /// set register
    ///
    /// copies the contents of register A into register C. (Input B is ignored.)
    Setr(Reg, Reg, Reg),
    /// set immediate
    ///
    /// stores value A into register C. (Input B is ignored.)
    Seti(Reg, Reg, Reg),

    /// greater-than immediate/register
    ///
    /// sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    Gtir(Reg, Reg, Reg),
    /// greater-than register/immediate
    ///
    /// sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
    Gtri(Reg, Reg, Reg),
    /// greater-than register/register
    ///
    /// sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
    Gtrr(Reg, Reg, Reg),

    /// equal immediate/register
    ///
    /// sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    Eqir(Reg, Reg, Reg),
    /// equal register/immediate
    ///
    /// sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    Eqri(Reg, Reg, Reg),
    /// equal register/register
    ///
    /// sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
    Eqrr(Reg, Reg, Reg),
}

impl Opcode {
    fn run(&self, state: &mut RegisterFile) {
        use self::Opcode::*;

        match *self {
            Addr(a, b, c) => state[c] = state[a] + state[b],
            Addi(a, b, c) => state[c] = state[a] + b,
            Mulr(a, b, c) => state[c] = state[a] * state[b],
            Muli(a, b, c) => state[c] = state[a] * b,
            Banr(a, b, c) => state[c] = state[a] & state[b],
            Bani(a, b, c) => state[c] = state[a] & b,
            Borr(a, b, c) => state[c] = state[a] | state[b],
            Bori(a, b, c) => state[c] = state[a] | b,
            Setr(a, _b, c) => state[c] = state[a],
            Seti(a, _b, c) => state[c] = a,
            Gtir(a, b, c) => state[c] = if a > state[b] { 1 } else { 0 },
            Gtri(a, b, c) => state[c] = if state[a] > b { 1 } else { 0 },
            Gtrr(a, b, c) => state[c] = if state[a] > state[b] { 1 } else { 0 },
            Eqir(a, b, c) => state[c] = if a == state[b] { 1 } else { 0 },
            Eqri(a, b, c) => state[c] = if state[a] == b { 1 } else { 0 },
            Eqrr(a, b, c) => state[c] = if state[a] == state[b] { 1 } else { 0 },
        }
    }

    fn every(raw_instruction: [usize; 4]) -> impl Iterator<Item = Self> {
        use self::Opcode::*;
        let [_, a, b, c] = raw_instruction;

        vec![
            Addr(a, b, c),
            Addi(a, b, c),
            Mulr(a, b, c),
            Muli(a, b, c),
            Banr(a, b, c),
            Bani(a, b, c),
            Borr(a, b, c),
            Bori(a, b, c),
            Setr(a, b, c),
            Seti(a, b, c),
            Gtir(a, b, c),
            Gtri(a, b, c),
            Gtrr(a, b, c),
            Eqir(a, b, c),
            Eqri(a, b, c),
            Eqrr(a, b, c),
        ].into_iter()
    }
}

struct Testcase {
    before: RegisterFile,
    raw_instruction: [usize; 4],
    after: RegisterFile,
}

impl Testcase {
    fn candidates<'a>(&'a self) -> impl Iterator<Item = Opcode> + 'a {
        Opcode::every(self.raw_instruction).filter(move |op| {
            let mut state = self.before;
            op.run(&mut state);
            state == self.after
        })
    }
}

#[test]
fn testcase_0() {
    let tc = Testcase {
        before: [3, 2, 1, 1],
        raw_instruction: [9, 2, 1, 2],
        after:  [3, 2, 2, 1],
    };

    assert_eq!(tc.candidates().count(), 3)
}

fn testcases<'a>(input: &'a str) -> impl Iterator<Item = Testcase> + 'a {
    fn digits<'a>(s: &'a str) -> impl Iterator<Item = usize> + 'a {
        s.split(|c: char| !c.is_digit(10)).flat_map(str::parse)
    }

    input.lines().filter(|l| !l.trim().is_empty()).tuples().map(|(b, i, a)| {
        let b: ArrayVec<_> = digits(b).collect();
        let i: ArrayVec<_> = digits(i).collect();
        let a: ArrayVec<_> = digits(a).collect();

        let before = b.into_inner().unwrap();
        let raw_instruction = i.into_inner().unwrap();
        let after = a.into_inner().unwrap();

        Testcase { before, raw_instruction, after }
    })
}
