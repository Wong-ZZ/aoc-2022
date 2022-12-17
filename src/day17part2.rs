use std::collections::{HashSet};

use crate::parser::read_file;

pub fn main() {
    // there is a repeating pattern starting at i == 1747 where height = 2773
    // every 1745 iterations after this, height increases by 2752
    // to find height at iteration n,
    // step 1: let x = 2773 + ((n-1747) / 1745) * 2752
    // step 2: let foo = (n-1747) % 1745
    // step 3: let y = (height at iteration (foo + 1747)) - 2773
    // answer = x + y
    // for n = 1000000000000, x = 1577077362325, foo = 1008, y = 4363 - 2773
    let n: i64 = 1000000000000;
    let result = ((n-1747) / 1745) * 2752i64 + solve(1747 + (n-1747) % 1745);
    println!("{result}");
}

fn solve(n: i64) -> i64 {
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
                .collect::<Vec<i64>>()
        })
        .unwrap();
    let mut filled_pos = HashSet::new();
    let mut max_y = 0;
    let mut jet_i = 0;
    for i in 0..n {
        let shape = match i % 5 {
            0 => vec![(0,0), (1, 0), (2, 0), (3, 0)],
            1 => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            2 => vec![(0, 0), (1, 0), (2,0), (2, 1), (2, 2)],
            3 => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            4 => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
            _ => panic!()
        };
        let mut x: i64 = 2;
        let mut y: i64 = max_y + 4;
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
    return max_y;
}
