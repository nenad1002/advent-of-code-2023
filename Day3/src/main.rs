use std::{fs::File};
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_vec = read_input();

    let mut res = 0;

    let m = input_vec.len();
    let n = input_vec[0].len();

    let directions: [[i32; 2]; 8] = [
        [0, 1],
        [1, 0],
        [0, -1],
        [-1, 0],
        [-1, -1],
        [1, 1],
        [-1, 1],
        [1, -1]
    ];

    for i in 0..m {
        let mut num = "".to_string();
        let mut valid = false;
        for j in 0..n{
            let element = input_vec[i].chars().collect::<Vec<_>>()[j];
            if element.is_digit(10) {
                num.push(element);

                for k in 0..directions.len() {
                    let new_i = (i as i32) + directions[k][0];
                    let new_j = (j as i32) + directions[k][1];
                    if check(&input_vec, new_i, new_j, m, n) {
                        valid = true;
                    }
                }
            }

            if !element.is_digit(10) || j == n-1 {
                if valid {
                    res += parse_int(num);
                    valid = false;
                }
                num = "".to_string();
            }
        }
    }   
    println!("{}", res);

}

fn check(matrix : &Vec<String>, i : i32, j : i32, m : usize, n : usize) -> bool {
    if i < 0 || j < 0 || i >= m as i32 || j >= n as i32 {
        return false;
    }

    let c = matrix[i as usize].chars().collect::<Vec<_>>()[j as usize];

    c != '.' && !c.is_digit(10)
}

fn parse_int(num: String) -> i32 {
    // Ignore potential errors
    num.parse::<i32>().unwrap()
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