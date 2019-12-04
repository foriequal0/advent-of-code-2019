use std::error::Error;
use std::io;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn Error + 'static>>;

fn get_masses<R: BufRead>(read: R) -> Result<Vec<u64>> {
    let mut result = Vec::new();
    for line in read.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let weight = line.parse::<u64>()?;
        result.push(weight);
    }
    Ok(result)
}

fn fuel_amount(mut weight: u64) -> u64 {
    let mut total = 0;
    while weight > 0 {
        let fuel = (weight / 3).saturating_sub(2);
        total += fuel;
        weight = fuel;
    }
    total
}

fn run<R: BufRead>(read: R) -> Result<u64> {
    Ok(get_masses(read)?.into_iter().map(fuel_amount).sum())
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let sum = run(stdin.lock())?;
    println!("fuels: {}", sum);
    Ok(())
}

#[test]
fn test() {
    let input = include_bytes!("../../input/01");
    assert_eq!(run(&input[..]).ok(), Some(4836845));
}
