use std::collections::{BTreeMap, BTreeSet};
use itertools::Itertools;
use arrayvec::ArrayVec;

static INPUT_PART_1: &str = include_str!("../input-part1.txt");
static INPUT_PART_2: &str = include_str!("../input-part2.txt");

fn main() {
    let mut multi_mapping: BTreeMap<_, BTreeSet<_>> = testcases(INPUT_PART_1)
        .map(|tc| {
            let code = tc.instruction_code();
            let possibilities = tc.candidates().map(|o| o.constructor()).collect();
            (code, possibilities)
        })
        .collect();

    let mut mapping = BTreeMap::new();

    while !multi_mapping.is_empty() {
        let code = multi_mapping
            .iter()
            .filter(|(_, possibilities)| possibilities.len() == 1)
            .map(|(&code, _)| code)
            .next()
            .expect("Couldn't find a definite mapping possibility");

        let possibilities = multi_mapping.remove(&code).expect("Code no longer present");
        let constructor = possibilities.into_iter().next().expect("Possibility is missing");

        mapping.insert(code, constructor);
        for possibilities in multi_mapping.values_mut() {
            possibilities.remove(&constructor);
        }
    }

    let program = compile(load(INPUT_PART_2), &mapping);
    let mut registers = RegisterFile::default();
    for opcode in program {
        opcode.run(&mut registers);
    }
    println!("Final registers: {:?}", registers);
}

fn compile<'a>(raw: impl Iterator<Item = RawInstruction> + 'a, mapping: &'a BTreeMap<usize, OpcodeFn>) -> impl Iterator<Item = Opcode> + 'a {
    raw.map(move |raw| {
        let [i, a, b, c] = raw;
        let args = (a, b, c);
        let constructor = mapping.get(&i).expect("Unknown instruction");
        constructor(args)
    })
}

type Reg = usize;
type RegisterFile = [Reg; 4];
type RawInstruction = [usize; 4];
type Args = (Reg, Reg, Reg);
type OpcodeFn = fn(Args) -> Opcode;

#[derive(Debug, Copy, Clone)]
enum Opcode {
    /// add register
    ///
    /// stores into register C the result of adding register A and register B.
    Addr(Args),
    /// add immediate
    ///
    /// stores into register C the result of adding register A and value B.
    Addi(Args),

    /// multiply register
    ///
    /// stores into register C the result of multiplying register A and register B.
    Mulr(Args),
    /// multiply immediate
    ///
    /// stores into register C the result of multiplying register A and value B.
    Muli(Args),

    /// bitwise AND register
    ///
    /// stores into register C the result of the bitwise AND of register A and register B.
    Banr(Args),
    /// bitwise AND immediate
    ///
    /// stores into register C the result of the bitwise AND of register A and value B.
    Bani(Args),

    /// bitwise OR register
    ///
    /// stores into register C the result of the bitwise OR of register A and register B.
    Borr(Args),
    /// bitwise OR immediate
    ///
    /// stores into register C the result of the bitwise OR of register A and value B.
    Bori(Args),

    /// set register
    ///
    /// copies the contents of register A into register C. (Input B is ignored.)
    Setr(Args),
    /// set immediate
    ///
    /// stores value A into register C. (Input B is ignored.)
    Seti(Args),

    /// greater-than immediate/register
    ///
    /// sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    Gtir(Args),
    /// greater-than register/immediate
    ///
    /// sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
    Gtri(Args),
    /// greater-than register/register
    ///
    /// sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
    Gtrr(Args),

    /// equal immediate/register
    ///
    /// sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    Eqir(Args),
    /// equal register/immediate
    ///
    /// sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    Eqri(Args),
    /// equal register/register
    ///
    /// sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
    Eqrr(Args),
}

impl Opcode {
    fn run(&self, state: &mut RegisterFile) {
        use self::Opcode::*;

        match *self {
            Addr((a, b, c)) => state[c] = state[a] + state[b],
            Addi((a, b, c)) => state[c] = state[a] + b,
            Mulr((a, b, c)) => state[c] = state[a] * state[b],
            Muli((a, b, c)) => state[c] = state[a] * b,
            Banr((a, b, c)) => state[c] = state[a] & state[b],
            Bani((a, b, c)) => state[c] = state[a] & b,
            Borr((a, b, c)) => state[c] = state[a] | state[b],
            Bori((a, b, c)) => state[c] = state[a] | b,
            Setr((a, _b, c)) => state[c] = state[a],
            Seti((a, _b, c)) => state[c] = a,
            Gtir((a, b, c)) => state[c] = if a > state[b] { 1 } else { 0 },
            Gtri((a, b, c)) => state[c] = if state[a] > b { 1 } else { 0 },
            Gtrr((a, b, c)) => state[c] = if state[a] > state[b] { 1 } else { 0 },
            Eqir((a, b, c)) => state[c] = if a == state[b] { 1 } else { 0 },
            Eqri((a, b, c)) => state[c] = if state[a] == b { 1 } else { 0 },
            Eqrr((a, b, c)) => state[c] = if state[a] == state[b] { 1 } else { 0 },
        }
    }

    fn every(raw_instruction: RawInstruction) -> impl Iterator<Item = Self> {
        use self::Opcode::*;
        let [_, a, b, c] = raw_instruction;
        let args = (a, b, c);
        vec![
            Addr(args),
            Addi(args),
            Mulr(args),
            Muli(args),
            Banr(args),
            Bani(args),
            Borr(args),
            Bori(args),
            Setr(args),
            Seti(args),
            Gtir(args),
            Gtri(args),
            Gtrr(args),
            Eqir(args),
            Eqri(args),
            Eqrr(args),
        ].into_iter()
    }

    fn constructor(&self) -> OpcodeFn {
        use self::Opcode::*;

        match *self {
            Addr(_) => Addr,
            Addi(_) => Addi,
            Mulr(_) => Mulr,
            Muli(_) => Muli,
            Banr(_) => Banr,
            Bani(_) => Bani,
            Borr(_) => Borr,
            Bori(_) => Bori,
            Setr(_) => Setr,
            Seti(_) => Seti,
            Gtir(_) => Gtir,
            Gtri(_) => Gtri,
            Gtrr(_) => Gtrr,
            Eqir(_) => Eqir,
            Eqri(_) => Eqri,
            Eqrr(_) => Eqrr,
        }
    }
}

struct Testcase {
    before: RegisterFile,
    raw_instruction: RawInstruction,
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

    fn instruction_code(&self) -> usize {
        let [i, _, _, _] = self.raw_instruction;
        i
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

fn load<'a>(input: &'a str) -> impl Iterator<Item = RawInstruction> + 'a {
    input.lines().filter(|l| !l.trim().is_empty()).map(|l| {
        let l: ArrayVec<_> = digits(l).collect();
        l.into_inner().unwrap()
    })
}

fn digits<'a>(s: &'a str) -> impl Iterator<Item = usize> + 'a {
    s.split(|c: char| !c.is_digit(10)).flat_map(str::parse)
}
