use std::io;

use crate::parser::read_file;

pub fn main() -> io::Result<()> {
    let lines = read_file(1, 1);
    let mut heap = std::collections::BinaryHeap::new();
    let mut temp = 0;

    for line in lines {
        let int_opt = line.unwrap().parse::<i32>();
        if int_opt.is_ok() {
            temp += int_opt.unwrap();
        } else {
            heap.push(-temp);
            if heap.len() > 3 {
                heap.pop();
            }
            temp = 0;
        }
    }

    let mut result = 0;
    for i in heap.iter() {
        result -= i;
    }
    println!("{}", result);

    Ok(())
}