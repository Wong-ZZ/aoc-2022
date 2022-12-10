use crate::parser::read_file;

pub fn main() {
    let mut cycle = 0;
    let mut x = 1;
    let mut grid = [['.'; 40]; 6];
    read_file(10, 1)
        .for_each(|l| {
            let line = l.unwrap();
            let mut i = line.split(' ').map(String::from);
            let op = i.next().unwrap();
            let num = i.next().map(|s| s.parse::<i32>().unwrap_or_else(|_| 0));
            match op.as_str() {
                "noop" => {
                    let row = cycle / 40;
                    let idx = cycle % 40;
                    if (x - idx as i32).abs() <= 1 {
                        grid[row][idx] = '#';
                    }
                    cycle += 1;
                },
                "addx" => {
                    let row = cycle / 40;
                    let idx = cycle % 40;
                    if (x - idx as i32).abs() <= 1 {
                        grid[row][idx] = '#';
                    }
                    cycle += 1;

                    let row = cycle / 40;
                    let idx = cycle % 40;
                    if (x - idx as i32).abs() <= 1 {
                        grid[row][idx] = '#';
                    }
                    cycle += 1;

                    x += num.unwrap();
                },
                _ => ()
            }
        });
    grid.iter().for_each(|r| println!("{:?}", r));
}
