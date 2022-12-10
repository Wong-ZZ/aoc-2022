use crate::parser::read_file;

pub fn main() {
    let mut cycle = 0;
    let mut result = 0;
    let mut x = 1;
    let checkpoints = (0..6).map(|i| 20 + i * 40).collect::<std::collections::HashSet<i32>>();
    let iter = read_file(10, 1)
        .for_each(|l| {
            let line = l.unwrap();
            let mut i = line.split(' ').map(String::from);
            let op = i.next().unwrap();
            let num = i.next().map(|s| s.parse::<i32>().unwrap_or_else(|_| 0));
            match op.as_str() {
                "noop" => {
                    cycle += 1;
                    if checkpoints.contains(&cycle) {
                        result += x * cycle;
                    }
                },
                "addx" => {
                    cycle += 1;
                    if checkpoints.contains(&cycle) {
                        result += x * cycle;
                    }
                    cycle += 1;
                    if checkpoints.contains(&cycle) {
                        result += x * cycle;
                    }
                    x += num.unwrap();
                },
                _ => ()
            }
        });
    println!("{result}")
}
