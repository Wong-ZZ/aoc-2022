use std::io;

use crate::parser::read_file;

pub fn main() -> io::Result<()> {
    let lines = read_file(2, 1);
    let mut results = 0;

    for line in lines {
        let input = line.unwrap();
        let input: Vec<&str> = input.split(' ').collect();
        match [input[0], input[1]] {
            ["A", "X"] => results += 3,
            ["A", "Y"] => results += 4,
            ["A", "Z"] => results += 8,
            ["B", "X"] => results += 1,
            ["B", "Y"] => results += 5,
            ["B", "Z"] => results += 9,
            ["C", "X"] => results += 2,
            ["C", "Y"] => results += 6,
            ["C", "Z"] => results += 7,
            [_,_] => results += 0
        }
    }
    
    println!("{}", results);

    Ok(())
}