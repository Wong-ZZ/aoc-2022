use crate::parser::read_file;


pub fn main() {
    let mut grid = read_file(22, 1)
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .filter(|v| !v.is_empty())
        .collect::<Vec<Vec<char>>>();

    let mut x: i32 = 0;
    let mut y = grid.get(0).unwrap().iter().enumerate().filter(|(_, c)| **c != ' ').map(|(i, _)| i as i32).next().unwrap();
    // let up = (-1, 0);
    // let down = (1, 0);
    // let left = (0, -1);
    // let right = (0, 1);
    // let dirs = [right, down, left, up];

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
    for c in instruction.iter() {
        match c {
            'L' | 'R' => {
                let to_move = num_moves.join("").parse::<i32>().unwrap();
                num_moves.clear();
                (0..to_move)
                    .for_each(|_| {
                        let (new_x, new_y, new_dir_i) = calc(x, y, curr_dir_i, &grid);
                        println!("before:({x},{y}),{curr_dir_i}");
                        println!("after:({new_x},{new_y}),{new_dir_i}");
        
                        match grid.get(new_x as usize).unwrap().get(new_y as usize).unwrap() {
                            '.' => {
                                x = new_x;
                                y = new_y;
                                curr_dir_i = new_dir_i;
                            },
                            '#' => (),
                            ' ' => {
                                panic!();
                            },
                            _ => panic!()
                        }
                    });
                if *c == 'L' && curr_dir_i == 0 {
                    curr_dir_i = 3
                } else if *c == 'L' {
                    curr_dir_i -= 1;
                } else {
                    curr_dir_i = (curr_dir_i + 1) % 4;
                }
            },
            _ => num_moves.push((*c).to_string())
        }
    }
    if !num_moves.is_empty() {
        let to_move = num_moves.join("").parse::<i32>().unwrap();
        num_moves.clear();
        (0..to_move)
            .for_each(|_| {
                let (new_x, new_y, new_dir_i) = calc(x, y, curr_dir_i, &grid);

                match grid.get(new_x as usize).unwrap().get(new_y as usize).unwrap() {
                    '.' => {
                        x = new_x;
                        y = new_y;
                        curr_dir_i = new_dir_i;
                    },
                    '#' => (),
                    ' ' => panic!(),
                    _ => panic!()
                }
            })
    }
    println!("{x},{y},{curr_dir_i}");
    // grid.iter().for_each(|r| println!("{:?}", r));
    println!("{}", 1000 * (x+1) + 4 * (y+1) + curr_dir_i as i32);
}

// _12
// _3_
// 45_
// 6__

// 1__
// 2_3
// __4
// _56
fn get_face(x: i32, y: i32) -> i32 {
    let x = x / 50;
    let y = y / 50;
    match (x, y) {
        (0, 1) => 1,
        (0, 2) => 2,
        (1, 1) => 3,
        (2, 0) => 4,
        (2, 1) => 5,
        (3, 0) => 6,

        (0, 0) => -1,
        (1, 0) => -2,
        (1, 2) => -3,
        (2, 2) => -4,
        (3, 1) => -5,
        (3, 2) => -6,

        _ => panic!()
    }
}

fn calc(x: i32, y: i32, curr_dir_i: usize, grid: &Vec<Vec<char>>) -> (i32, i32, usize) {
    let up = (-1, 0);
    let down = (1, 0);
    let left = (0, -1);
    let right = (0, 1);
    let dirs = [right, down, left, up];
    let curr_dir = dirs[curr_dir_i];

    let m = grid.len() as i32;
    let n = grid.get(0).unwrap().len() as i32;
    
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
    let curr_face = get_face(x, y);
    let next_face = get_face(new_x, new_y);
    let offset_x = x % 50;
    let offset_y = y % 50;
    match (curr_face, next_face) {
        // left of 1 -> left of 4 
        (1, -1) => (100 + (49 - offset_x), 0, 0),
        // top of 1 -> left of 6
        (1, -5) => (150 + offset_y , 0, 0),
        // top of 2 -> bottom of 6
        (2, -6) => (199, offset_y, 3),
        // right of 2 -> right of 5 
        (2, -1) => (100 + (49 - offset_x) , 99,  2),
        // bottom of 2 -> right of 3
        (2, -3) => (50 + offset_y , 99, 2),
        // left of 3 -> top of 4
        (3, -2) => (100, offset_x, 1),
        // right of 3 -> bottom of 2
        (3, -3) => (49, 100 + offset_x, 3),
        // top of 4 -> left of 3
        (4, -2) => (50 + offset_y, 50, 0),
        // left of 4 ->  left of 1
        (4, -4) => (49 - offset_x, 50, 0),
        // right of 5 -> right of 2
        (5, -4) => (49 - offset_x, 149, 2),
        // bottom of 5 -> right of 6
        (5, -5) => (150 + offset_y, 49, 0),
        // left of 6 -> top of 1
        (6, -6) => (0, 50 + offset_x, 1),
        // right of 6 -> bottom of 5
        (6, -5) => (149, 50 + offset_x, 3),
        // bottom of 6 -> top of 2
        (6, -1) => (0, 100 + offset_y, 1),
        _ => (new_x, new_y, curr_dir_i)
    }

}