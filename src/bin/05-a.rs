use std::convert::TryFrom;
use std::error::Error;
use std::io;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn Error + 'static>>;

fn get_program<R: BufRead>(read: R) -> Result<Vec<i64>> {
    let mut inputs = Vec::new();
    for value in read.split(b',') {
        let value = value?;
        let value = std::str::from_utf8(&value)?;
        let value = value.trim();
        if value.is_empty() {
            continue;
        }
        let number = value.parse::<i64>()?;
        inputs.push(number)
    }

    Ok(inputs)
}

#[derive(Debug)]
enum Instruction {
    Halt,
    Add { a: Param, b: Param, c: usize },
    Mul { a: Param, b: Param, c: usize },
    Input(usize),
    Output(Param),
}

impl Instruction {
    fn from_slice(slice: &[i64]) -> Option<Instruction> {
        let command = slice.first().and_then(|x| u64::try_from(*x).ok())?;
        let op = (command % 100) as u64;
        let get_param = |idx: usize| {
            let mode = (command / 10_u64.pow(idx as u32 + 2)) % 10;
            let param = *slice.get(idx + 1)?;
            let res = match mode {
                0 => Param::Position(usize::try_from(param).ok()?),
                1 => Param::Immediate(param),
                _ => return None,
            };
            Some(res)
        };

        let res = match op {
            99 => Instruction::Halt,
            1 => Instruction::Add {
                a: get_param(0)?,
                b: get_param(1)?,
                c: get_param(2)?.position()?,
            },
            2 => Instruction::Mul {
                a: get_param(0)?,
                b: get_param(1)?,
                c: get_param(2)?.position()?,
            },
            3 => Instruction::Input(get_param(0)?.position()?),
            4 => Instruction::Output(get_param(0)?),
            _ => return None,
        };
        Some(res)
    }

    fn pc_offset(&self) -> usize {
        match self {
            Instruction::Halt => 1,
            Instruction::Add { .. } => 4,
            Instruction::Mul { .. } => 4,
            Instruction::Input(..) => 2,
            Instruction::Output(..) => 2,
        }
    }
}

#[derive(Debug)]
enum Param {
    Position(usize),
    Immediate(i64),
}

impl Param {
    fn position(&self) -> Option<usize> {
        match self {
            Param::Position(addr) => Some(*addr),
            Param::Immediate(_) => None,
        }
    }

    fn get(&self, memory: &[i64]) -> Option<i64> {
        match self {
            Param::Position(addr) => memory.get(*addr).cloned(),
            Param::Immediate(x) => Some(*x),
        }
    }
}

fn execute(mut memory: Vec<i64>, mut input: &[i64], output: &mut Vec<i64>) -> Option<i64> {
    let mut pc = 0;
    loop {
        let slice = memory.get(pc..)?;
        let inst = Instruction::from_slice(slice)?;
        match &inst {
            Instruction::Halt => break,
            Instruction::Add { a, b, c } => {
                *memory.get_mut(*c)? = a.get(&memory)? + b.get(&memory)?;
            }
            Instruction::Mul { a, b, c } => {
                *memory.get_mut(*c)? = a.get(&memory)? * b.get(&memory)?;
            }
            Instruction::Input(pos) => {
                *memory.get_mut(*pos)? = *input.first()?;
                input = input.get(1..)?;
            }
            Instruction::Output(param) => {
                output.push(param.get(&memory)?);
            }
        }
        pc += inst.pc_offset();
    }
    Some(memory[0])
}

fn run<R: BufRead>(read: R, input: i64) -> Result<i64> {
    let program = get_program(read)?;
    let mut output = Vec::new();
    execute(program, &[input], &mut output).ok_or("program error")?;
    if output.iter().take(output.len() - 1).any(|x| *x != 0) {
        Err("test fail")?
    } else {
        Ok(output.last().cloned().ok_or("no output")?)
    }
}

const INPUT: i64 = 1;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let code = run(stdin.lock(), INPUT)?;
    println!("code: {}", code);
    Ok(())
}

#[test]
fn test() {
    let input = include_bytes!("../../input/05");
    assert_eq!(run(&input[..], INPUT).ok(), Some(7566643));
}
