use std::collections::{HashSet};

use crate::parser::read_file;

pub fn main() {
    let jet = read_file(17, 1)
        .next()
        .unwrap()
        .map(|s| {
            s
                .chars()
                .map(|c| match c {
                    '>' => 1,
                    '<' => -1,
                    _ => panic!()
                })
                .collect::<Vec<i32>>()
        })
        .unwrap();
    let mut filled_pos = HashSet::new();
    let mut max_y = 0;
    let mut jet_i = 0;
    for i in 0..2022 {
        let shape = match i % 5 {
            0 => vec![(0,0), (1, 0), (2, 0), (3, 0)],
            1 => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            2 => vec![(0, 0), (1, 0), (2,0), (2, 1), (2, 2)],
            3 => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            4 => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
            _ => panic!()
        };
        let mut x = 2;
        let mut y = max_y + 4;
        loop {
            let jet_dir = jet.get(jet_i).unwrap();
            let can_push = shape.iter().all(|(x2, y2)| {
                let new_x = x2 + x + jet_dir;
                let new_y = y2 + y;
                new_x < 7 &&  new_x >= 0 && !filled_pos.contains(&(new_x, new_y))
            });
            if can_push {
                x += jet_dir;
            }
            jet_i = (jet_i + 1) % jet.len();
            let can_drop = shape.iter().all(|(x2, y2)| {
                let new_x = x2 + x;
                let new_y = y2 + y - 1;
                return new_y > 0 && !filled_pos.contains(&(new_x, new_y))
            });
            if can_drop {
                y -= 1;
            } else {
                shape.iter().for_each(|(x2, y2)| {
                    let new_x = x2 + x;
                    let new_y = y2 + y;
                    filled_pos.insert((new_x, new_y));
                    max_y = std::cmp::max(new_y, max_y);
                });
                break;
            }
        }
    }
    // let mut grid = [['.'; 7]; 20];
    // filled_pos.iter().for_each(|(x, y)| grid[20-*y][*x as usize] = '#');
    // grid.iter()
    //     .for_each(|r| {
    //         print!("[");
    //         r.iter().for_each(|c| print!("{c}"));
    //         println!("]");
    //     });
    println!("{max_y}");
}
