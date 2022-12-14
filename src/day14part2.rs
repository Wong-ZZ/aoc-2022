use crate::parser::read_file;

pub fn main() {
    const LEN_X: usize = 1000;
    let mut grid = [[false;LEN_X];300];
    let mut max_y = 0;
    read_file(14, 1)
        .map(|l| l.unwrap().split(" -> ").map(String::from).collect::<Vec<String>>())
        .for_each(|coords| {
            let coords = coords
                .iter()
                .map(|s|{
                    let pos = s.split(',').collect::<Vec<&str>>();
                    return (pos.get(0).unwrap().parse::<i32>().unwrap(), pos.get(1).unwrap().parse::<i32>().unwrap())
                })
                .collect::<Vec<(i32, i32)>>();
            for idx in 0..coords.len()-1 {
                let (mut x, mut y) = coords.get(idx).unwrap();
                let (next_x, next_y) = coords.get(idx+1).unwrap();
                let dir_x = if next_x - x == 0 {
                    0
                } else if next_x - x < 0 {
                    -1
                } else {
                    1
                };
                let dir_y = if next_y - y == 0 {
                    0
                } else if next_y - y < 0 {
                    -1
                } else {
                    1
                };

                max_y = std::cmp::max(max_y, y);
                grid[y as usize][x as usize] = true;
                while (x, y) != (*next_x, *next_y) {
                    x += dir_x;
                    y += dir_y;
                    grid[y as usize][x as usize] = true;
                }
            }
        });
    max_y += 2;
    grid[max_y as usize] = [true;LEN_X];

    let mut i = 0;
    loop {
        let mut x = 500usize;
        let mut y = 0usize;
        if grid[y][x] {
            println!("{i}");
            break;
        }

        loop {
            if !grid[y+1][x] {
                y += 1;
            } else if !grid[y+1][x-1] {
                y += 1;
                x -= 1;
            } else if !grid[y+1][x+1] {
                y += 1;
                x += 1;
            } else {
                grid[y][x] = true;
                break;
            }
        }
        i += 1;
    }
}
