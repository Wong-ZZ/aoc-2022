use crate::parser::read_file;


pub fn main() {
    let mut grid = read_file(22, 1)
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .filter(|v| !v.is_empty())
        .collect::<Vec<Vec<char>>>();

    let mut x: i32 = 0;
    let mut y = grid.get(0).unwrap().iter().filter(|c| **c != ' ').enumerate().map(|(i, _)| i as i32).next().unwrap();
    let up = (-1, 0);
    let down = (1, 0);
    let left = (0, -1);
    let right = (0, 1);
    let dirs = [right, down, left, up];

    let mut curr_dir_i = 0;
    let instruction = grid.pop().unwrap();
    let max_len = grid.iter().map(|r| r.len()).reduce(|x, y| std::cmp::max(x, y)).unwrap();
    println!("{max_len}");
    for i in 0..grid.len() {
        let row = grid.get_mut(i).unwrap();
        for _ in 0..(max_len-row.len()) {
            row.push(' ');
        }
    }

    let mut num_moves: Vec<String> = vec![];
    let m = grid.len() as i32;
    let n = grid.get(0).unwrap().len() as i32;
    for c in instruction.iter() {
        match c {
            'L' | 'R' => {
                let to_move = num_moves.join("").parse::<i32>().unwrap();
                num_moves.clear();
                let curr_dir = dirs[curr_dir_i];
                if *c == 'L' && curr_dir_i == 0 {
                    curr_dir_i = 3
                } else if *c == 'L' {
                    curr_dir_i -= 1;
                } else {
                    curr_dir_i = (curr_dir_i + 1) % 4;
                }
                (0..to_move)
                    .for_each(|_| {
                        let mut new_x = x;
                        let mut new_y = y;
                        new_x = match new_x == 0 && curr_dir.0 == -1 {
                            true => m - 1,
                            false => (new_x + curr_dir.0) % m
                        };
                        new_y = match new_y == 0 && curr_dir.1 == -1 {
                            true => n - 1,
                            false => (new_y + curr_dir.1) % n
                        };
                        loop {
                            match grid.get(new_x as usize).unwrap().get(new_y as usize).unwrap() {
                                '.' => {
                                    x = new_x;
                                    y = new_y;
                                    break;
                                },
                                '#' => break,
                                ' ' => {
                                    new_x = match new_x == 0 && curr_dir.0 == -1 {
                                        true => m - 1,
                                        false => (new_x + curr_dir.0) % m
                                    };
                                    new_y = match new_y == 0 && curr_dir.1 == -1 {
                                        true => n - 1,
                                        false => (new_y + curr_dir.1) % n
                                    };
                                },
                                _ => panic!()
                            }
                        }
                    })
            },
            _ => num_moves.push((*c).to_string())
        }
    }
    if !num_moves.is_empty() {
        let to_move = num_moves.join("").parse::<i32>().unwrap();
        num_moves.clear();
        let curr_dir = dirs[curr_dir_i];
        (0..to_move)
            .for_each(|_| {
                let mut new_x = x;
                let mut new_y = y;
                new_x = match new_x == 0 && curr_dir.0 == -1 {
                    true => m - 1,
                    false => (new_x + curr_dir.0) % m
                };
                new_y = match new_y == 0 && curr_dir.1 == -1 {
                    true => n - 1,
                    false => (new_y + curr_dir.1) % n
                };
                loop {
                    match grid.get(new_x as usize).unwrap().get(new_y as usize).unwrap() {
                        '.' => {
                            x = new_x;
                            y = new_y;
                            break;
                        },
                        '#' => break,
                        ' ' => {
                            new_x = match new_x == 0 && curr_dir.0 == -1 {
                                true => m - 1,
                                false => (new_x + curr_dir.0) % m
                            };
                            new_y = match new_y == 0 && curr_dir.1 == -1 {
                                true => n - 1,
                                false => (new_y + curr_dir.1) % n
                            };
                        },
                        _ => panic!()
                    }
                }
            })
    }
    println!("{x},{y},{curr_dir_i}");
    // grid.iter().for_each(|r| println!("{:?}", r));
    println!("{}", 1000 * (x+1) + 4 * (y+1) + curr_dir_i as i32);
}
