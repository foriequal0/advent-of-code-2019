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

fn stack(layers: &[&[u8]]) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.resize(layers[0].len(), 0);
    for layer in layers.iter().cloned().rev() {
        for (i, p) in layer.iter().cloned().enumerate() {
            match p {
                b'0' => buffer[i] = b'0',
                b'1' => buffer[i] = b'1',
                _ => {}
            }
        }
    }
    buffer
}

fn into_image(layer: &[u8], w: usize, h: usize) -> String {
    let mut result = String::new();
    for line in layer.chunks_exact(w).map(std::str::from_utf8) {
        result.push_str(line.unwrap());
        result.push('\n');
    }
    result
}

fn run<R: BufRead>(read: R, input: i64) -> Result<String> {
    let data = get_data(read)?;
    let layers = split_layers(&data, 25, 6);
    let stacked = stack(&layers);
    let image = into_image(&stacked, 25, 6);
    Ok(image)
}

const INPUT: i64 = 1;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let image = run(stdin.lock(), INPUT)?;
    println!("image: \n{}", image);
    Ok(())
}

#[test]
fn test() {
    let input = include_bytes!("../../input/08");
    assert_eq!(
        run(&input[..], INPUT).unwrap(),
        "\
1111 1   1111  1    1  1 
   1 1   11  1 1    1  1 
  1   1 1 111  1    1111 
 1     1  1  1 1    1  1 
1      1  1  1 1    1  1 
1111   1  111  1111 1  1 
"
    );
}
