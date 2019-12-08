use std::error::Error;
use std::io;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn Error + 'static>>;

fn get_data<R: BufRead>(mut read: R) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    read.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn split_layers(data: &[u8], w: usize, h: usize) -> Vec<&[u8]> {
    data.chunks_exact(w * h).collect()
}

fn count(layer: &[u8], value: u8) -> usize {
    layer.iter().filter(|x| **x == value).count()
}

fn find_min_0_value(layers: &[&[u8]]) -> usize {
    let mut min = std::usize::MAX;
    let mut min_index = 0;
    for (i, layer) in layers.iter().enumerate() {
        let zeros = count(layer, b'0');
        if zeros < min {
            min = zeros;
            min_index = i;
        }
    }
    let ones = count(layers[min_index], b'1');
    let twos = count(layers[min_index], b'2');
    ones * twos
}

fn run<R: BufRead>(read: R, input: i64) -> Result<usize> {
    let data = get_data(read)?;
    let layers = split_layers(&data, 25, 6);
    let value = find_min_0_value(&layers);
    Ok(value)
}

const INPUT: i64 = 1;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let max = run(stdin.lock(), INPUT)?;
    println!("value: {}", max);
    Ok(())
}

#[test]
fn test() {
    let input = include_bytes!("../../input/08");
    assert_eq!(run(&input[..], INPUT).unwrap(), 2480);
}
