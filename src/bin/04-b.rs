use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::ops::RangeInclusive;

type BoxedError = Box<dyn Error + 'static>;
type Result<T> = std::result::Result<T, BoxedError>;

fn get_range<R: BufRead>(read: R) -> Result<RangeInclusive<u32>> {
    let mut split = read.split(b'-');
    let from = split.next().expect("No to")?;
    let from = String::from_utf8(from)?.trim().parse::<u32>()?;
    let to = split.next().expect("No to")?;
    let to = String::from_utf8(to)?.trim().parse::<u32>()?;
    Ok(from..=to)
}

fn is_password(x: u32) -> bool {
    let mut buf = [0u8; 6];
    if write!(&mut buf[..], "{}", x).is_err() || buf.contains(&0) {
        return false;
    }
    let mut double = false;
    let mut streak = 1;
    for (a, b) in buf[0..buf.len() - 1].iter().zip(&buf[1..]) {
        if a > b {
            return false;
        }
        if a == b {
            streak += 1;
        } else {
            if streak == 2 {
                double = true;
            }
            streak = 1;
        }
    }
    if streak == 2 {
        double = true;
    }
    double
}

fn run<R: BufRead>(read: R) -> Result<(usize)> {
    Ok(get_range(read)?.filter(|x| is_password(*x)).count())
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let result = run(stdin.lock())?;
    println!("result: {}", result);
    Ok(())
}

#[test]
fn test() {
    let input = include_bytes!("../../input/04");
    assert_eq!(run(&input[..]).ok(), Some(334));
}
