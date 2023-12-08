use std::{fs::File};
use std::io::{self, BufRead};
use std::path::Path;

const NUMBERS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];
fn main() {

    let input_vec = read_input();
    let mut res = 0;

    for line in input_vec.iter() {
        let line = line.as_bytes();
        let num1 = (0 .. line.len()).find_map(|index| parse_line_v2(line, index)).unwrap();
        let num2 = (0 .. line.len()).rev().find_map(|index| parse_line_v2(line, index)).unwrap();

        res += 10 * num1 + num2;
    }

    println!("{}", res);
   
}

fn parse_line_v2(line: &[u8], i: usize) -> Option<usize> {
    line[i]
        .is_ascii_digit()
        .then_some((line[i] - b'0') as usize)
        .or(NUMBERS
            .iter()
            .enumerate()
            .find(|(_, name)| line[i..].starts_with(name))
            .map(|(num, _)| num + 1))
}

fn parse_line_v1(line: &[u8], i: usize) -> Option<usize> {
    line[i]
        .is_ascii_digit()
        .then_some((line[i] - b'0') as usize)
}

fn read_input() -> Vec<String> {
    let mut res = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                res.push(line);
            }
        }
    }

    res
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}