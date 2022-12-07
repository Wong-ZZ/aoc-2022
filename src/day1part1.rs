use std::io;

use crate::parser::read_file;

pub fn main() -> io::Result<()> {
    let lines = read_file(1, 1);
    let mut result = 0;
    let mut temp = 0;

    for line in lines {
        let int_opt = line.unwrap().parse::<i32>();
        if int_opt.is_ok() {
            temp += int_opt.unwrap();
        } else {
            result = std::cmp::max(result, temp);
            temp = 0;
        }
    }
    println!("{}", result);

    Ok(())
}