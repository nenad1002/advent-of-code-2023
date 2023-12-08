use std::{fs::File};
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_vec = read_input();

    let mut res = 0;

    for i in 0..input_vec.len() {
        let line = input_vec[i].as_bytes();
        res += process_line_v2(line);
    }

    println!("{}", res);
   
}

fn process_line_v2(line: &[u8]) -> u32 {
    let mut rgb = [0, 0, 0];

    line.splitn(2, |it| it == &b':')
    .nth(1)
    .unwrap()
    .split(|it| it == &b',' || it == &b';')
    .for_each(|item| {
        let i = match item[1..].splitn(2, |it| it == &b' ').nth(1).unwrap() {
            b"red" => 0usize,
            b"green" => 1,
            b"blue" => 2,
            _ => unreachable!(),
        };
        rgb[i] = rgb[i].max(atoi::atoi(&item[1..]).unwrap());
    });

    rgb[0] * rgb[1] * rgb[2]
}

fn process_line_v1(line: &[u8]) -> Option<bool> {
    let mut rgb = [0, 0, 0];

    line.splitn(2, |it| it == &b':')
    .nth(1)
    .unwrap()
    .split(|it| it == &b',' || it == &b';')
    .all(|item| {
        let i = match item[1..].splitn(2, |it| it == &b' ').nth(1).unwrap() {
            b"red" => 0usize,
            b"green" => 1,
            b"blue" => 2,
            _ => unreachable!(),
        };
        rgb[i] = rgb[i].max(atoi::atoi(&item[1..]).unwrap());
        rgb[i] <= 12 + i as u32
    })
    .then_some(true)
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