use std::error::Error;
use std::io;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn Error + 'static>>;

fn get_program<R: BufRead>(read: R) -> Result<Vec<u64>> {
    let mut inputs = Vec::new();
    for value in read.split(b',') {
        let value = value?;
        let value = std::str::from_utf8(&value)?;
        let value = value.trim();
        if value.is_empty() {
            continue;
        }
        let number = value.parse::<u64>()?;
        inputs.push(number)
    }

    Ok(inputs)
}

fn restore(mut program: Vec<u64>, noun: u64, verb: u64) -> Vec<u64> {
    program[1] = noun;
    program[2] = verb;
    program
}

fn execute(mut program: Vec<u64>) -> Option<u64> {
    let mut pc = 0;
    while program[pc] != 99 {
        if let &[op, a, b, c] = program.get(pc..pc + 4)? {
            let a = *program.get(a as usize)?;
            let b = *program.get(b as usize)?;
            let c = program.get_mut(c as usize)?;
            match op {
                1 => *c = a + b,
                2 => *c = a * b,
                _ => return None,
            }
            pc += 4;
        } else {
            return None;
        }
    }
    Some(program[0])
}

fn find(program: Vec<u64>, target: u64) -> Result<(u64, u64)> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let restored = restore(program.clone(), noun, verb);
            if execute(restored) == Some(target) {
                return Ok((noun, verb));
            }
        }
    }
    Err("Not found")?
}

fn run<R: BufRead>(read: R, target: u64) -> Result<(u64, u64)> {
    let program = get_program(read)?;
    find(program, target)
}

const TARGET: u64 = 19690720;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let (noun, verb) = run(stdin.lock(), TARGET)?;
    println!(
        "noun: {}, verb: {}, 100 * noun + verb: {}",
        noun,
        verb,
        100 * noun + verb
    );
    Ok(())
}

#[test]
fn test() {
    let input = include_bytes!("../../input/02");
    assert_eq!(run(&input[..], TARGET).ok(), Some((59, 36)));
}
